<template>
  <div class="container" @scroll="handleScroll">
    <div
      class="item"
      v-for="(image, index) in images"
      :key="index"
      @click.stop.prevent="contextMenuShow = false"
      @scroll="handleScroll"
    >
      <img
        :src="image.thumbnail"
        alt=""
        @contextmenu="(e) => handleContextMenu(e, image)"
      />
    </div>

    <ul ref="contextMenu" class="context-menu" v-show="contextMenuShow">
      <li @click="handleDownload">下载图片</li>
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
      query: { page: 1, keyword: "" },
      images: [],
      currentImage: null,
      contextMenuShow: false,
      handleContextMenu: async (e, image) => {
        e.preventDefault();

        const { x, y } = e;
        dataMap.contextMenuShow = true;
        const context = instance.refs.contextMenu;
        context.style.left = `${x - 10}px`;
        context.style.top = `${y - 10}px`;
        context.style.position = "fixed";

        dataMap.currentImage = image;
      },
      handleDownload: async () => {
        console.log("start download");
        const { currentImage } = dataMap;
        if (currentImage) {
          const { source } = currentImage;
          const local_path = await invoke("download_image", { source });
          console.log(local_path);
          alert("下载成功");
        } else {
          alert("请选择图片");
        }
        dataMap.contextMenuShow = false;
      },
      handleScroll: async (e) => {
        console.log(e);
      },
    });

    onMounted(async () => {
      const response = await invoke("get_images", dataMap.query);
      dataMap.images = response;
      console.log(dataMap.images);
      window.addEventListener("scroll", async () => {
        const windowHeight =
          document.documentElement.clientHeight || document.body.clientHeight;
        const scrollTop =
          document.documentElement.scrollTop || document.body.scrollTop;
        const documentHeight =
          document.documentElement.scrollHeight || document.body.scrollHeight;
        if (windowHeight + scrollTop >= documentHeight) {
          console.log("到底了");
          dataMap.query.page++;
          const images = await invoke("get_images", dataMap.query);
          console.log(images);
          dataMap.images.push(...images);
        }
      });
    });

    const setBackground = async () => {
      dataMap.contextMenuShow = false;
      if (dataMap.currentImage != null) {
        const response = await invoke("set_background", {
          source: dataMap.currentImage.source,
          thumbnail: dataMap.currentImage.thumbnail,
        });
        console.log(response);
        alert("设置成功");
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
  column-count: 3;
  column-gap: 5;
}
.item {
  margin: 10px;
  border: 1px solid #ccc;
  border-radius: 5px;
  padding: 3px;
  cursor: pointer;
}
.item img {
  width: 90%;
  opacity: 0.9;
  transition: all 0.3s;
}

.item img:hover {
  opacity: 1;
  width: 100%;
}

.context-menu {
  background: white;
  border: 1px solid #ccc;
  border-radius: 5px;
  padding: 10px;
  position: fixed;
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
