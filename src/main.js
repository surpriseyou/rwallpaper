import { createApp } from "vue";
import App from "./App.vue";
import ElementPlus from "element-plus";
import "element-plus/dist/index.css";
import VueLazyLoad from "vue-lazyload";
import { isRegistered, register } from "@tauri-apps/api/globalShortcut";
// import { invoke } from "@tauri-apps/api";
import { appWindow } from "@tauri-apps/api/window";

// Register global shortcuts
const shortcut = "CommandOrControl+Shift+Enter";
isRegistered(shortcut).then((registered) => {
  if (!registered) {
    console.log(`Registering global shortcut: ${shortcut}`);
  }
});
register("CommandOrControl+Shift+Enter", async () => {
  await appWindow.center();
  await appWindow.show();
}).then(() => {
  console.log("Registered");
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

app.use(ElementPlus);

app.mount("#app");
