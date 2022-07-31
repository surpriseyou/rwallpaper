<template>
  <image-gallery> </image-gallery>
  <el-backtop :right="20" :bottom="20" :visibility-height="300" />
</template>

<script>
import { onMounted, ref } from "@vue/runtime-core";
import ImageGallery from "./components/ImageGallery.vue";
import { invoke } from "@tauri-apps/api";
import { appWindow } from "@tauri-apps/api/window";

export default {
  name: "App",
  components: {
    ImageGallery,
  },
  setup() {
    onMounted(async () => {
      console.log("App mounted");
      await appWindow.onCloseRequested(async (event) => {
        await appWindow.hide();
        event.preventDefault();
      });
    });
    const getImages = () => {
      invoke("get_images").then((response) => console.log(response));
    };
    const counter = ref(0);
    return {
      msg: "Welcome to Your Vue.js App",
      getImages,
      counter,
    };
  },
};
</script>

<style>
html,
body,
#app {
  height: 100%;
  margin: 0;
}
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}
</style>
