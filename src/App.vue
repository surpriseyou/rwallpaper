<template>
  <image-gallery> </image-gallery>
</template>

<script>
import { onMounted, ref } from "@vue/runtime-core";
import ImageGallery from "./components/ImageGallery.vue";
import { invoke } from "@tauri-apps/api";
export default {
  name: "App",
  components: {
    ImageGallery,
  },
  setup() {
    onMounted(() => {
      console.log("App mounted");
      invoke("hello", { name: "World" })
        // `invoke` returns a Promise
        .then((response) => console.log(response));
    });
    const getImages = () => {
      invoke("get_images")
        // `invoke` returns a Promise
        .then((response) => console.log(response));
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
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}
</style>
