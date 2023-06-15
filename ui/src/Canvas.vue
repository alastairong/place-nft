<template>
    <div v-if="loading" style="display: flex; flex: 1; align-items: center; justify-content: center">
      <mwc-circular-progress indeterminate></mwc-circular-progress>
    </div>
  
    <div class="container" v-else style="display: flex; flex-direction: column">
      <div class="grid">
        <div
          v-for="(cell, index) in grid"
          :key="index"
          class="cell"
          :style="{ backgroundColor: cell.toString() }"
          @click="placePixel(index)"
        ></div>
      </div>
      <div class="color-picker">
        <div v-for="color in colors" :key="color.toString()"
            class="color-swatch"
            :style="{ backgroundColor: color.toString() }"
            @click="selectedColor = color"
            :class="{ active: color === selectedColor }">
        </div>
      </div>
      <div v-if="finished">
        <div class="overlay"></div> <!--if this doesn't cover enough, move this to top level and add v-if condition -->
          <Minter></Minter>
      </div>
    </div>
  
  </template>
  
  <script lang="ts">
  import Minter from './Minter.vue';
  import { defineComponent, inject, toRaw, ComputedRef } from 'vue';
  import { Interface } from './place_nft/interface';
  import { AppAgentClient } from '@holochain/client';
  import { Snapshot, Placement, DestructuredPlacement, PlaceSignal } from './place_nft/types';
  import { packPlacement, updateGrid, color2index, COLOR_PALETTE } from './place_nft/utils';
  import '@material/mwc-circular-progress';
  // TODO: Placements outside of a snapshot are not currently rendered
  const GAME_START_TIME = 1686833278; // Must be updated to match DNA timestamp
  const BUCKET_DURATION = 60 * 1; // 1 minutes
  const BUCKETS_PER_HOUR = 60 * 60 / BUCKET_DURATION;
  const HOURS_OF_GAMEPLAY = 1;
  const GAME_END_TIME = GAME_START_TIME + HOURS_OF_GAMEPLAY * BUCKET_DURATION * BUCKETS_PER_HOUR;
  
  export default defineComponent({
    components: {
      Minter
    },
    data(): { grid: String[]; selectedColor: String; clock: number; currentBucket: number; latestSnapshot: Snapshot | undefined; latestSnapshotBucket: number; placementsSinceLatestSnapshot: Array<Placement>; loading: boolean; error: any; timer: any; colors: String[]; finished: boolean} {
      return {
        grid: Array(16384).fill("#ffffff"), // Create a grid of 100x100 cells and set their background color to white
        selectedColor: "#ffffff", // Initialize the selected color to white
        clock: 0,
        currentBucket: 0,
        latestSnapshot: undefined,
        latestSnapshotBucket: -1,
        placementsSinceLatestSnapshot: [],
        loading: true,
        error: undefined,
        timer: undefined,
        colors: COLOR_PALETTE,
        finished: false
      }
    },
    created() {
      this.timer = setInterval(this.updateData, 10000) // we check for updates every 10 seconds
    }, 
    destroyed() {
      clearInterval(this.timer); // clear timer when component is destroyed
    },
    async mounted() {
      this.calculateCurrentBucket();
      await this.loadInitialData();
      toRaw(this.client).on('signal', signal => {
        if (signal.zome_name !== 'posts') return; 
        const payload = signal.payload as PlaceSignal;
        if (payload.type !== 'EntryCreated') return;
        if (payload.app_entry.type !== 'Placement') return;
        console.log("RECEIVED: " + payload.app_entry);
        if (this.placementsSinceLatestSnapshot) this.placementsSinceLatestSnapshot.push(payload.app_entry);
      });
    },
    methods: {
      calculateCurrentBucket() {
          const now = new Date();
          const timeInSeconds = Math.round(now.getTime() / 1000); 
          this.clock = Math.min(timeInSeconds, GAME_END_TIME);
          this.currentBucket = Math.floor((this.clock - GAME_START_TIME) / BUCKET_DURATION);
          console.log("Current bucket: " + this.currentBucket);
      },

      async loadInitialData() {
          try {
            console.log("Loading initial data at bucket " + this.currentBucket);
            const maybeSnapshot = await this.happ.getSnapshotAt(this.currentBucket); // get latest snapshot
            
            if (maybeSnapshot == null) {
              console.log("No snapshot found at bucket " + this.currentBucket + ", trying to catch up...");
              await this.catchUpToCurrentSnapshot();  // if there's no snapshot, find the most recent one and publish old snapshots until now
            } else {
              this.latestSnapshot = maybeSnapshot;
              this.latestSnapshotBucket = this.currentBucket;
            }
            this.placementsSinceLatestSnapshot = await this.happ.getPlacementsAt(this.currentBucket); // get placements at the current bucket
            this.loading = false;
            this.finished = Date.now() > GAME_END_TIME * 1000;
          } catch (e) {
              console.log(e);
          }
      },

      async catchUpToCurrentSnapshot() {
        console.log("Calling catchUpToCurrentSnapshot")
        let bucket = this.currentBucket;
        console.log("bucket is " + bucket);
        // iterate backwards through buckets until we find a snapshot
        while (bucket > 0) {
          bucket = bucket - 1;
          console.log("checking if snapshot exists at bucket " + bucket + "...")
          let snapshot = await this.happ.getSnapshotAt(bucket);
          if (snapshot) {
            this.latestSnapshot = snapshot;
            this.latestSnapshotBucket = this.currentBucket;
            break;
          }
        };
        // if we're at the first bucket and there's no snapshot, create a starting snapshot
        if (bucket == 0) {
          console.log("No snapshot found at bucket 0, publishing starting snapshot...");
          const maybeSnapshot = await this.happ.publishStartingSnapshot();

          if (maybeSnapshot !== null) {
            this.latestSnapshot = maybeSnapshot;
            this.latestSnapshotBucket = 0;
            console.log("Published starting snapshot: " + this.latestSnapshot);
          } else {
            console.log("Error publishing starting snapshot");
          }
          
        };
        
        // iterate forward through buckets to publish any missing snapshots
        while (bucket < this.currentBucket) {
          console.log("publishing snapshot at bucket " + bucket);
          const maybeSnapshot = await this.happ.publishSnapshotAt(bucket);
          if (maybeSnapshot !== null) {
            this.latestSnapshot = maybeSnapshot;
            this.latestSnapshotBucket = bucket;
            console.log("Published snapshot: " + this.latestSnapshot);
          } else {
            console.log("Error publishing snapshot");
          }
          bucket = bucket + 1;
        };

        if (bucket == this.currentBucket) {
          await this.tryPublish();
        }
      },

      async updateData() {
        console.log("Updating data at bucket " + this.currentBucket + " at " + Date.now());
          // update clock and check if we've moved to a new bucket
          const now = new Date();
          const timeInSeconds = Math.round(now.getTime() / 1000); 
          this.finished = timeInSeconds > GAME_END_TIME;

          this.clock = Math.min(timeInSeconds, GAME_END_TIME);
          const bucket = Math.floor((this.clock - GAME_START_TIME) / BUCKET_DURATION);
          
          // move to new bucket if changed
          if (bucket != this.currentBucket) {
            console.log("Moving to new bucket " + bucket);
            this.currentBucket = bucket;
          } else {
            console.log("Still at bucket " + bucket);
          }
          
          if (this.latestSnapshotBucket < this.currentBucket) {
            try {    
              let newSnapshot = await this.happ.getSnapshotAt(bucket);
              if (newSnapshot) {
                this.latestSnapshot = newSnapshot;
                this.placementsSinceLatestSnapshot = await this.happ.getPlacementsAt(bucket);
                // console.log("Placements since latest snapshot: " + JSON.stringify(this.placementsSinceLatestSnapshot));
              } else {
                await this.tryPublish();
              }
            } catch (e) {
              console.log(e);
            }
          }
          
      },

      async placePixel(index: number) { 
        console.log("Calling placePixel")
        let destructuredPlacement: DestructuredPlacement = {
          x: index % 128,
          y: Math.floor(index / 128),
          colorIndex: color2index(this.selectedColor.toString()),
        }
        // console.log("Checking if already placed at bucket " + this.currentBucket + "...")
        const alreadyPlaced = await this.happ.alreadyPlaced(this.clock) // this check also means that if we haven't gotten a snapshot for the new bucket we can't place
        if (!alreadyPlaced) {
          console.log("Placing pixel at bucket " + this.currentBucket + "...")
          const placementAction = await this.happ.placePixel(destructuredPlacement);
          this.placementsSinceLatestSnapshot.push(packPlacement(destructuredPlacement));
          // this.grid[index] = this.selectedColor; // Update the color of the selected cell in the grid
          // console.log("grid at index " + index + " is " + this.grid[index]);
        } else {
          console.log("Already placed at bucket " + this.currentBucket + "...")
        }
      },

      async tryPublish() {
        console.log("Calling tryPublish")
        let rank = await this.happ.getAuthorRank(Math.max(this.currentBucket - 1, 0)); // rank is based on actions in the last bucket
        if (rank == 0) {
          rank = 10;
        }
        console.log("CurrentBucket is " + this.currentBucket)
        console.log("Rank is " + rank)
        let secondsInBucket = (this.clock - GAME_START_TIME) - (this.currentBucket * BUCKET_DURATION); 
        console.log("Seconds in bucket is " + secondsInBucket)
        if ((rank - 1) <= Math.floor(secondsInBucket/2) || secondsInBucket > 20 || this.currentBucket * BUCKET_DURATION + GAME_START_TIME >= GAME_END_TIME) {
          console.log("Publishing snapshot at bucket " + this.currentBucket + "...")
          const snapshot = await this.happ.publishSnapshotAt(this.currentBucket);
          if (snapshot) {
            this.latestSnapshot = snapshot;
            this.placementsSinceLatestSnapshot = [];
          }
        } else {
          console.log("Rank too low / not enough time elapsed to publish")
          return
        }
      },
    },
    // TODO: reset the placements array when the snapshot array is updated?
    watch: {
      latestSnapshot(newSnapshot) {
        this.placementsSinceLatestSnapshot = []; 
        this.grid = updateGrid(newSnapshot.imageData, []); // unpack grid data
      },

      placementsSinceLatestSnapshot(newPlacements) {
          if (newPlacements.length > 0) {
          let baseImageData: Uint8Array;
          if (this.latestSnapshot) {
            baseImageData = this.latestSnapshot.imageData;
          } else {
            baseImageData = new Uint8Array(128 * 128);
          }
          
          this.grid = updateGrid(baseImageData, newPlacements); // unpack grid data
        }
      },

      finished(newValue) { 
        if (newValue) {
          clearInterval(this.timer); // cancel the timer when game is finished
        }
      }
    },

    setup() {
      const client = (inject('client') as ComputedRef<AppAgentClient>).value;
      const happ = (inject('placeInterface') as ComputedRef<Interface>).value;
      return {
        client,
        happ,
      };
    },
  })

  </script>
  
  <style>
    .container {
      display: flex;
      flex-direction: column;
      align-items: center;
    }
    
    .grid {
      display: grid;
      grid-template-columns: repeat(128, 1fr);
      grid-template-rows: repeat(128, 1fr);
      grid-gap: 0px;
      width: 1000px;
      height: 1000px;
      margin-bottom: 20px;
      border: 1px solid #ddd;
    }
    
    .cell {
      background-color: #ffffff;
      cursor: pointer;
      border: 1px solid #ddd;
    }
    
    .color-picker {
      display: flex;
      align-items: center;
      position: fixed; 
      top: 0; 
      width: 50%; 
      background-color: #fff; 
      z-index: 1; 
    }
    
    .color-swatch {
      width: 20px; /* set width of input */
      height: 20px; /* set height of input */
      margin: 0; /* remove any margin */
      padding: 0; /* remove any padding */
      border: none; /* remove any border */
      box-shadow: 0 0 0 1px #000; /* add a black border effect */
    }

    .overlay {
      position: fixed;
      top: 0;
      left: 0;
      height: 100%;
      width: 100%;
      background-color: rgba(0, 0, 0, 0.5);
      z-index: 1000;
    }

    .modal {
      /* your modal styles */
      position: fixed;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      z-index: 1002; /* higher than .overlay */
      background-color: white;
      padding: 20px;
      border: 1px solid #ccc;
      width: 80%;
    }

    w3m-modal {
      z-index: 1003; /* higher than .modal */
    }
  </style>
  