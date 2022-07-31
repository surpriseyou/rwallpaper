<template>
  <div class="query-container">
    <el-select
      size="default"
      placeholder="搜索源"
      v-model="query.source"
      @change="handleQuery"
    >
      <el-option
        v-for="(source, index) in sources"
        :key="index"
        :value="source"
        :label="source"
      ></el-option>
    </el-select>
    <el-input
      class="input-text"
      v-model="query.keyword"
      placeholder="请输入关键字"
      @keyup.enter="handleQuery"
      :prefix-icon="Search"
      clearable
    ></el-input>
    <button type="button" class="search-btn" @click="handleQuery">
      <img src="../assets/search.png" alt="../assets/image.png" />
      搜索
    </button>
  </div>
  <div class="container" @scroll="handleScroll">
    <div class="image-group" v-for="(group, index) in images" :key="index">
      <div
        class="item"
        v-for="(image, index) in group.items"
        :key="index"
        @click.stop.prevent="contextMenuShow = false"
      >
        <img
          v-lazy="image.thumbnail"
          alt=""
          @contextmenu="(e) => handleContextMenu(e, image)"
        />
      </div>

      <div class="divider">
        第 <strong>{{ group.page }}</strong> 页
      </div>
    </div>

    <ul ref="contextMenu" class="context-menu" v-show="contextMenuShow">
      <li @click="handleDownload">下载图片</li>
      <li @click="setBackground">设置背景</li>
    </ul>
  </div>
  <div class="empty-container" v-if="images.length === 0">
    <img src="../assets/empty.png" alt="" />
    <p>没有搜索到相关图片</p>
  </div>
  <div class="loading-container" v-if="loading">
    <div class="loading-img">
      <svg
        t="1658066948726"
        class="icon"
        viewBox="0 0 1024 1024"
        version="1.1"
        xmlns="http://www.w3.org/2000/svg"
        p-id="1373"
        width="200"
        height="200"
      >
        <path
          d="M381.838313 867.335265c12.074748 12.074748 18.419107 27.526332 18.419107 45.945438s-6.037374 33.973019-18.419107 46.559408c-12.074748 12.58639-27.321675 18.726092-45.433796 18.726092s-33.154392-6.242031-45.433796-18.726092c-12.074748-12.58639-18.419107-27.935645-18.419107-46.559408 0-18.419107 6.037374-33.768362 18.419107-45.945438 12.074748-12.074748 27.321675-18.521435 45.433796-19.340062C354.516638 848.609174 369.45658 854.953533 381.838313 867.335265z"
          p-id="1374"
        ></path>
        <path
          d="M497.981013 890.666134c-16.065554 0-29.470571 5.62806-40.317378 16.679524-10.846807 11.051464-16.270211 24.763466-16.270211 41.340662 0 16.270211 5.423404 29.879884 16.270211 40.726691 10.846807 10.846807 24.354152 16.270211 40.726691 16.270211s29.879884-5.423404 40.726691-16.270211 16.270211-24.354152 16.270211-40.726691-5.423404-29.982212-16.270211-41.340662C528.474868 896.294194 514.660538 890.666134 497.981013 890.666134z"
          p-id="1375"
        ></path>
        <path
          d="M190.279604 733.182772c-20.874988 0.61397-38.168482 7.879285-51.778155 21.386629-13.609673 13.507345-20.465674 30.596183-20.465674 51.061857s6.958329 37.554512 20.465674 51.061857 30.596183 20.465674 51.061857 20.465674 37.554512-6.958329 51.061857-20.465674c13.507345-13.507345 20.465674-30.596183 20.465674-51.061857s-6.958329-37.554512-20.465674-51.061857C227.117818 741.062057 210.540622 733.796742 190.279604 733.182772z"
          p-id="1376"
        ></path>
        <path
          d="M225.787549 273.216748c26.503048-0.61397 48.605976-10.02818 66.308784-27.730988 17.702808-17.702808 26.707705-40.010393 26.707705-66.820426s-9.004897-49.015289-26.707705-66.820426c-17.702808-17.702808-39.805736-26.503048-66.308784-26.503048s-48.605976 9.004897-66.308784 26.707705-26.707705 40.010393-26.707705 66.820426 9.004897 49.015289 26.707705 66.820426C177.181573 263.495553 199.284501 272.50045 225.787549 273.216748z"
          p-id="1377"
        ></path>
        <path
          d="M89.076846 486.059758c24.354152-0.61397 44.615169-9.209553 61.192365-25.684421 16.270211-16.270211 24.558809-36.735885 24.558809-61.192365 0-24.354152-8.083941-44.615169-24.558809-61.192365-16.270211-16.270211-36.940542-24.558809-61.49935-24.558809s-45.22914 8.083941-61.49935 24.558809c-16.474868 16.679524-24.558809 36.940542-24.558809 61.397022 0 24.354152 8.083941 44.615169 24.558809 61.192365C43.438393 476.850205 64.108724 485.34346 89.076846 486.059758z"
          p-id="1378"
        ></path>
        <path
          d="M170.530229 627.886879c-0.61397-22.921555-8.595583-41.852303-23.637854-56.894574s-33.768362-22.512241-56.280604-22.512241-41.340662 7.469971-56.280604 22.512241-22.512241 33.768362-22.512241 56.280604 7.469971 41.340662 22.512241 56.280604 33.768362 22.512241 56.280604 22.512241 41.340662-7.469971 56.280604-22.512241C161.934646 668.715899 169.916259 650.092136 170.530229 627.886879z"
          p-id="1379"
        ></path>
        <path
          d="M964.905366 519.316478c-3.78615-4.195463-9.004897-6.242031-15.246927-6.242031s-11.460777 2.046567-15.656241 6.242031c-4.195463 4.195463-6.242031 9.41421-6.242031 15.656241s2.046567 11.460777 6.242031 15.656241 9.41421 6.242031 15.656241 6.242031c6.242031 0 11.256121-2.046567 15.246927-6.242031 3.78615-4.195463 5.832717-9.41421 5.832717-15.656241S968.589188 523.61427 964.905366 519.316478z"
          p-id="1380"
        ></path>
        <path
          d="M927.760168 619.495953c-8.288598 0-15.246927 2.660538-20.874988 8.288598s-8.288598 12.58639-8.288598 20.874988 2.660538 15.246927 8.288598 20.874988 12.58639 8.288598 20.874988 8.288598 15.246927-2.660538 20.465674-8.288598 7.879285-12.58639 7.879285-20.874988-2.660538-15.246927-7.879285-20.874988C942.904767 622.258819 936.048766 619.495953 927.760168 619.495953z"
          p-id="1381"
        ></path>
        <path
          d="M780.611972 811.463975c-12.58639 0.61397-22.716898 5.01409-30.903168 13.098031-7.879285 7.879285-12.074748 18.419107-12.074748 30.903168 0 12.58639 3.990806 22.921555 12.074748 31.312481 7.879285 8.288598 18.419107 12.58639 30.903168 12.58639s22.921555-4.195463 31.312481-12.58639c8.288598-8.288598 12.58639-18.726092 12.58639-31.312481s-4.195463-22.716898-12.58639-30.903168C803.533527 816.375737 793.198361 812.077945 780.611972 811.463975z"
          p-id="1382"
        ></path>
        <path
          d="M655.464375 872.963326c-13.916658 0-25.684421 4.809433-35.507944 14.632957-9.823524 9.823524-14.632957 21.693614-14.632957 36.019586 0 14.223643 4.809433 26.298391 14.632957 36.019586 9.823524 9.823524 21.693614 14.632957 36.019586 14.632957 14.223643 0 26.093734-4.809433 35.507944-14.632957 9.41421-9.823524 14.223643-21.693614 14.223643-36.019586 0-14.223643-4.809433-26.298391-14.223643-36.019586C682.274408 877.772759 670.097332 872.963326 655.464375 872.963326z"
          p-id="1383"
        ></path>
        <path
          d="M871.377236 722.745278c-9.823524 0.61397-18.21445 4.297792-24.968122 11.051464-6.958329 6.753672-10.437494 15.246927-10.437494 25.684421s3.581493 19.135405 10.437494 26.093734c6.958329 6.958329 15.451584 10.437494 25.684421 10.437494 10.02818 0 18.521435-3.581493 25.684421-10.437494 6.958329-6.958329 10.437494-15.656241 10.437494-26.093734s-3.581493-18.930748-10.437494-25.684421C890.61497 727.145398 881.81473 723.461577 871.377236 722.745278z"
          p-id="1384"
        ></path>
      </svg>
    </div>
  </div>
</template>

<script>
import {
  getCurrentInstance,
  onMounted,
  onBeforeMount,
  reactive,
  toRefs,
} from "@vue/runtime-core";
import { invoke } from "@tauri-apps/api";
import { Search } from "@element-plus/icons-vue";
import { ElNotification } from "element-plus";
export default {
  name: "ImageGallery",
  setup() {
    const instance = getCurrentInstance();
    const dataMap = reactive({
      query: { page: 1, keyword: "", source: "" },
      images: [],
      sources: [],
      currentImage: null,
      contextMenuShow: false,
      loading: false,
      canLoadMore: true, // 是否可以加载更多, 如果一次获取图片少于“pageSize”张, 就不能加载更多, 反之可以加载更多
      pageSize: 24, // 每页显示的图片数量
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
        dataMap.contextMenuShow = false;
        const { currentImage } = dataMap;
        if (currentImage) {
          const { source } = currentImage;
          const local_path = await invoke("download_image", { source });
          console.log(local_path);
          ElNotification({
            title: "提醒",
            message: "图片下载成功！",
            type: "success",
          });
        } else {
          ElNotification({
            title: "错误",
            message: "下载图片失败！",
            type: "error",
          });
        }
        dataMap.contextMenuShow = false;
      },
      handleQuery: async () => {
        dataMap.loading = true;
        dataMap.images = [];
        const { query } = dataMap;
        query.page = 1;
        const images = await invoke("get_images", query);
        if (images && images.length > 0) {
          dataMap.images.push({ page: 1, items: images });
        }

        if (!images || images.length < dataMap.pageSize) {
          dataMap.canLoadMore = false;
        } else {
          dataMap.canLoadMore = true;
        }

        dataMap.loading = false;
      },
    });

    onBeforeMount(async () => {});

    onMounted(async () => {
      const sources = await invoke("get_image_sources");
      dataMap.sources = sources;
      if (sources.length > 0) dataMap.query.source = sources[0];

      const response = await invoke("get_images", dataMap.query);
      dataMap.images.push({ page: 1, items: response });

      invoke("close_splashscreen");

      window.addEventListener("scroll", async () => {
        if (dataMap.loading) return;

        const windowHeight =
          document.documentElement.clientHeight || document.body.clientHeight;
        const scrollTop =
          document.documentElement.scrollTop || document.body.scrollTop;
        const documentHeight = document.body.scrollHeight;

        console.log(windowHeight, scrollTop, documentHeight);

        if (windowHeight + scrollTop >= documentHeight) {
          dataMap.loading = true;
          if (dataMap.images.length > 0) {
            dataMap.query.page++;
          } else {
            dataMap.query.page = 1;
          }

          const images = await invoke("get_images", dataMap.query);
          if (images && images.length > 0) {
            dataMap.images.push({ page: dataMap.query.page, items: images });
          }
          if (!images || images.length < dataMap.pageSize) {
            dataMap.canLoadMore = false;
          } else {
            dataMap.canLoadMore = true;
          }
          console.log(dataMap.canLoadMore);
          dataMap.loading = false;
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
        ElNotification({
          title: "提醒",
          message: "背景设置成功！",
          type: "success",
        });
      }
    };
    return {
      ...toRefs(dataMap),
      setBackground,
      Search,
    };
  },
};
</script>

<style>
.container {
  /* column-count: 3;
  column-gap: 5; */
  margin-top: 40px;
}
.image-group {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  grid-template-rows: repeat(3, 1fr);
  position: relative;
}

.divider {
  width: 100%;
  height: 20px;
  background-color: whitesmoke;
  display: flex;
  justify-content: center;
  align-items: center;
  position: absolute;
  bottom: -10px;
}
.item {
  margin: 10px;
  border: 1px solid #ccc;
  border-radius: 5px;
  padding: 3px;
  cursor: pointer;
}
.item img {
  width: 100%;
  opacity: 0.9;
  transition: all 0.3s;
}

.item img:hover {
  opacity: 1;
  transform: translateY(-3px);
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

.query-container {
  height: 30px;
  display: flex;
  justify-content: left;
  /* align-items: center; */
  box-shadow: 0 0 3px 3px lightgrey;
  position: fixed;
  top: 0;
  width: 100%;
  z-index: 100;
  background: white;
  padding: 3px;
}

.select-source {
  outline: none;
  border: none;
  width: fit-content;
  min-width: 100px;
  padding: 3px;
  appearance: none;
}

.source-option {
  border-radius: 0;
  border-bottom: 1px solid #ccc;
  padding: 3px;
  cursor: pointer;
  height: 24px;
}

.input-text {
  height: 30px;
  border: none;
  padding: 0 10px;
  flex: 1;
  outline: none;
}
.search-btn {
  width: 100px;
  height: 30px;
  border: none;
  padding: 0 10px;
  cursor: pointer;
  margin-left: 5px;
  margin-right: 10px;
  display: flex;
  justify-content: center;
  align-items: center;
  opacity: 0.9;
  transition: all 0.3s;
  background: cadetblue;
  color: white;
}
.search-btn img {
  width: 20px;
  margin-right: 3px;
  transition: all 0.3s;
}
.search-btn:hover {
  opacity: 1;
  border-radius: 3px;
}
.search-btn:hover img {
  /* margin-right: 10px;
  align-self: flex-start; */
  justify-self: flex-start;
}
.loading-container {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.5);
  z-index: 999;
  display: flex;
  justify-content: center;
  align-items: center;
}

/** loading img keyframes */
@keyframes loading {
  0% {
    /** rotate center */
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

.loading-img svg {
  width: 50px;
  height: 50px;
  transform-origin: 50% 50%;
  /* animation: loading 1s infinite linear; */
  animation-iteration-count: 10;
  animation-name: loading;
  animation-duration: 2s;
  animation-timing-function: ease-in-out;
}

.empty-container {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
  height: 100%;
}
.empty-container img {
  width: 150px;
}
</style>

<style>
/**
* 这里是修改自定义的样式，select的高度比input高了2px
*/
.el-input.el-input--default.el-input--suffix {
  height: 30px;
}
</style>