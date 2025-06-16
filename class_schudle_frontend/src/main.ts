import { createApp } from "vue";
import App from "./App.vue";
import ElementPlus from 'element-plus';
import 'element-plus/dist/index.css';
import * as ElementPlusIconsVue from '@element-plus/icons-vue';
import { themeStore } from './stores/themeStore';
import './styles/theme.css';

const app = createApp(App);

app.use(ElementPlus);

// 注册所有图标
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, component);
}

// 初始化主题系统
themeStore.init();

app.mount("#app");
