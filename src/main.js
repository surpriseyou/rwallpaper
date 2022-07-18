import { createApp } from "vue";
import App from "./App.vue";
import VueLazyLoad from "vue-lazyload";

const loadingImage = require("./assets/loading.png");
const errorImage = require("./assets/error.png");

const app = createApp(App);

app.use(VueLazyLoad, {
  preLoad: 1.3,
  error: errorImage,
  loading: loadingImage,
  attempt: 1,
});

app.mount("#app");
