import { createApp } from "vue";
import App from "./App.vue";
import VueLazyLoad from "vue-lazyload";
import { isRegistered, register } from "@tauri-apps/api/globalShortcut";
import { invoke } from "@tauri-apps/api";

// Register global shortcuts
const shortcut = "CommandOrControl+Shift+Enter";
isRegistered(shortcut).then((registered) => {
  if (!registered) {
    register("CommandOrControl+Shift+Enter", async () => {
      await invoke("show_main_window");
    }).then(() => {
      console.log("Registered");
    });
  }
});

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
