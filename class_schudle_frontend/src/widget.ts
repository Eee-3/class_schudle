import { createApp } from "vue";
import DesktopScheduleWidget from "./components/DesktopScheduleWidget.vue";
import { themeStore } from './stores/themeStore';
import './styles/theme.css';

const app = createApp(DesktopScheduleWidget);

// 初始化主题系统
themeStore.init();

app.mount("#app");
