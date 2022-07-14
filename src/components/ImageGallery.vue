<template>
  <div class="container">
    <div class="item" v-for="(image, index) in images" :key="index">
      <img
        :src="image.thumbnail"
        alt=""
        @contextmenu="(e) => handleContextMenu(e, image)"
      />
    </div>

    <ul ref="contextMenu" class="context-menu" v-show="contextMenuShow">
      <li>下载图片</li>
      <li @click="setBackground">设置背景</li>
    </ul>
  </div>
</template>

<script>
import {
  getCurrentInstance,
  onMounted,
  reactive,
  toRefs,
} from "@vue/runtime-core";
import { invoke } from "@tauri-apps/api";
export default {
  name: "ImageGallery",
  setup() {
    const instance = getCurrentInstance();
    const dataMap = reactive({
      images: [],
      currentImage: null,
      contextMenuShow: false,
      handleContextMenu: async (e, image) => {
        console.log(e);
        e.preventDefault();

        const { x, y } = e;
        dataMap.contextMenuShow = true;
        const context = instance.refs.contextMenu;
        context.style.left = `${x - 10}px`;
        context.style.top = `${y - 10}px`;

        dataMap.currentImage = image;
      },
    });

    onMounted(async () => {
      const response = await invoke("get_images");
      dataMap.images = response;
      console.log(dataMap.images);
    });

    const setBackground = async () => {
      dataMap.contextMenuShow = false;
      if (dataMap.currentImage != null) {
        console.log(dataMap.currentImage);
        const response = await invoke("set_background", {
          source: dataMap.currentImage.source,
          thumbnail: dataMap.currentImage.thumbnail,
        });
        console.log(response);
      }
    };
    return {
      ...toRefs(dataMap),
      setBackground,
    };
  },
};
</script>

<style>
.container {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  margin-top: 20px;
  flex-direction: row;
  align-items: center;
}
.item {
  width: 200px;
  height: 200px;
  margin: 10px;
  border: 1px solid #ccc;
  border-radius: 5px;
  display: flex;
  justify-content: center;
  align-items: center;
}
.context-menu {
  background: white;
  border: 1px solid #ccc;
  border-radius: 5px;
  padding: 10px;
  position: absolute;
  box-shadow: 0px 0px 5px gray;
  width: 150px;
}
.context-menu li {
  list-style: none;
  line-height: 40px;
}
.context-menu li:hover {
  background: whitesmoke;
  cursor: pointer;
}
</style>
