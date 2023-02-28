<template>
    <div v-if="loading" style="display: flex; flex: 1; align-items: center; justify-content: center">
      <mwc-circular-progress indeterminate></mwc-circular-progress>
    </div>
  
    <div v-else style="display: flex; flex-direction: column">
      <div class="grid">
        <div
          v-for="(cell, index) in grid"
          :key="index"
          class="cell"
          :style="{ color: cell.toString() }"
          @click="placePixel(index)"
        ></div>
        <div class="color-picker">
          <div v-for="color in colors" :key="color.toString()"
              class="color-swatch"
              :style="{ backgroundColor: color.toString() }"
              @click="selectedColor = color"
              :class="{ active: color === selectedColor }">
          </div>
        </div>
      </div>
    </div>
  
  </template>
  
  <script lang="ts">
  import { defineComponent, inject, ComputedRef } from 'vue';
  import { Interface } from './place_nft/place/interface';
  import { AppAgentClient } from '@holochain/client';
  import { Snapshot, Placement, DestructuredPlacement } from './place_nft/place/types';
  import { packPlacement, snapshotToGrid, color2index, COLOR_PALETTE } from './place_nft/place/utils';
  import '@material/mwc-circular-progress';

  const GAME_START_TIME = 1673631207; // Must be updated to match DNA timestamp
  
  export default defineComponent({
    data(): { grid: String[]; selectedColor: String; clock: number; currentBucket: number; latestSnapshot: Snapshot | undefined; placementsSinceLatestSnapshot: Array<Placement>; loading: boolean; error: any; timer: any; colors: String[];} {
      return {
        grid: Array(16384).fill("#ffffff"), // Create a grid of 100x100 cells and set their background color to white
        selectedColor: "#ffffff", // Initialize the selected color to white
        clock: 0,
        currentBucket: 0,
        latestSnapshot: undefined,
        placementsSinceLatestSnapshot: [],
        loading: true,
        error: undefined,
        timer: undefined,
        colors: COLOR_PALETTE
      }
    },
    created() {
      this.timer = setInterval(this.updateData, 5000) // we check for updates every 5 seconds
    }, 
    destroyed() {
      clearInterval(this.timer); // clear timer when component is destroyed
    },
    async mounted() {
      this.calculateCurrentBucket();
      await this.loadInitialData();
    },
    methods: {
      calculateCurrentBucket() {
          const now = new Date();
          const timeInSeconds = Math.round(now.getTime() / 1000); 
          this.clock = timeInSeconds;
          this.currentBucket = Math.floor(timeInSeconds / (60 * 5));
      },

      async loadInitialData() {
          try {
            this.latestSnapshot = await this.happ.getSnapshotAt(this.currentBucket); // get latest snapshot

            if (!this.latestSnapshot) {
              await this.catchUpToCurrentSnapshot();  // if there's no snapshot, find the most recent one and publish old snapshots until now
            }
            this.placementsSinceLatestSnapshot = await this.happ.getPlacementsAt(this.currentBucket); // get placements at the current bucket
            this.loading = false;
          } catch (e) {
              console.log(e);
          }
      },

      async catchUpToCurrentSnapshot() {
        let bucket = this.currentBucket;
        // iterate backwards through buckets until we find a snapshot
        while (bucket > 0) {
          bucket = bucket - 1;
          let snapshot = await this.happ.getSnapshotAt(bucket);
          if (snapshot) {
            this.latestSnapshot = snapshot;
            break;
          } 
          // if we're at the first bucket and there's no snapshot, create a starting snapshot
          if (bucket == 0) {
            this.latestSnapshot = await this.happ.publishStartingSnapshot();
          }
        };
        // iterate forward through buckets to publish any missing snapshots
        while (bucket < this.currentBucket) {
          this.latestSnapshot = await this.happ.publishSnapshotAt(bucket);
          bucket = bucket + 1;
        };

        if (bucket == this.currentBucket) {
          await this.tryPublish();
        }
      },

      // In this function, we only update the bucket once we have a new snapshot. This keeps the bucket
      // in sync with the snapshot, and acts as a flag for having a latest snapshot.
      async updateData() {
          // update clock and check if we've moved to a new bucket
          const now = new Date();
          const timeInSeconds = Math.round(now.getTime() / 1000); 
          this.clock = timeInSeconds;
          const bucket = Math.floor(timeInSeconds / (60 * 5));
          if (this.currentBucket < bucket) { // if we've moved to a new bucket
            try {
              // See if there's a new snapshot
              let newSnapshot = await this.happ.getSnapshotAt(bucket);
              if (newSnapshot) {
                this.latestSnapshot = newSnapshot;
                this.placementsSinceLatestSnapshot = await this.happ.getPlacementsAt(bucket);
                this.currentBucket = bucket;
              } else {
                await this.tryPublish();
              }
            } catch (e) {
              console.log(e);
            }
        }
      },

      async placePixel(index: number) { 
        let destructuredPlacement: DestructuredPlacement = {
          x: index % 128,
          y: Math.floor(index / 128),
          colorIndex: color2index(this.selectedColor.toString()),
        }
        const alreadyPlaced = await this.happ.alreadyPlaced(this.currentBucket) // this check also means that if we haven't gotten a snapshot for the new bucket we can't place
        if (!alreadyPlaced) {
          const placementAction = await this.happ.placePixel(destructuredPlacement);
          this.placementsSinceLatestSnapshot.push(packPlacement(destructuredPlacement));
        }
        this.grid[index] = this.selectedColor; // Update the color of the selected cell in the grid
      },

      async tryPublish() {
        let rank = await this.happ.getAuthorRank(this.currentBucket);
        if (rank == 0) {
          rank = 10;
        }

        let secondsInBucket = (this.clock - GAME_START_TIME)/(5 * 60) - (this.currentBucket * (5 * 60)); 

        if (rank <= (secondsInBucket)/10 || secondsInBucket > 150) {
          const snapshot = await this.happ.publishSnapshotAt(this.currentBucket);
          if (snapshot) {
            this.latestSnapshot = snapshot;
            this.placementsSinceLatestSnapshot = [];
          }
        } else {
          return
        }
      },
    },

    watch: {
      latestSnapshot(newSnapshot) {
        this.grid = snapshotToGrid(newSnapshot.imageData); // unpack grid data
      },
    },

    setup() {
      const client = (inject('client') as ComputedRef<AppAgentClient>).value;
      const placeInterface = new Interface(client);
      return {
        happ: placeInterface,
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
      grid-gap: 1px;
      width: 500px;
      height: 500px;
      margin-bottom: 20px;
    }
    
    .cell {
      background-color: #ffffff;
      cursor: pointer;
    }
    
    .input {
      display: flex;
      align-items: center;
    }
    
    label {
      margin-right: 10px;
    }
  </style>
  