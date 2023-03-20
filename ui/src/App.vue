<template>
  <div>
    <div v-if="loading">
      <mwc-circular-progress indeterminate></mwc-circular-progress>
    </div>
    <div v-else>
      <div id="content">
        <Canvas style="margin-bottom: 16px"></Canvas>
      </div>
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent, computed } from 'vue';
import { AppWebsocket, ActionHash, AppAgentClient, AppAgentWebsocket } from '@holochain/client';
import { Interface } from './place_nft/place/interface';
import '@material/mwc-circular-progress';
import Canvas from './Canvas.vue';


export default defineComponent({
  components: {
    Canvas
  },
  data(): {
    client: AppAgentClient | undefined;
    placeInterface: Interface | undefined;
    loading: boolean;
  } {
    return {
      client: undefined,
      placeInterface: undefined,
      loading: true,
    };
  },
  async mounted() {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    this.client = await AppAgentWebsocket.connect('', 'place-nft');
    this.placeInterface = new Interface(this.client); 
    this.loading = false;
  },
  provide() {
    return {
      client: computed(() => this.client),
      placeInterface: computed(() => this.placeInterface)
    };
  },
});
</script>
