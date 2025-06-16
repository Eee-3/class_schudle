<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { ElMessage, ElMessageBox } from "element-plus";
import ScheduleDisplay from "./components/ScheduleDisplay.vue";
import SettingsPanel from "./components/SettingsPanel.vue";
import { themeStore } from "./stores/themeStore";

const currentView = ref<'schedule' | 'settings'>('schedule');
const widgetVisible = ref(false);

function handleSwitchToSettings() {
  currentView.value = 'settings';
}

// 切换桌面小组件
async function toggleWidget() {
  try {
    const visible = await invoke<boolean>('toggle_widget');
    widgetVisible.value = visible;
    ElMessage.success(visible ? '桌面小组件已显示' : '桌面小组件已隐藏');
  } catch (error) {
    console.error('切换桌面小组件失败:', error);
    ElMessage.error('操作失败');
  }
}

// 退出应用
async function quitApp() {
  try {
    await ElMessageBox.confirm(
      '确定要退出应用吗？',
      '退出确认',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning',
      }
    );

    await invoke('quit_app');
  } catch (error) {
    if (error !== 'cancel') {
      console.error('退出应用失败:', error);
      ElMessage.error('退出失败');
    }
  }
}

// 主题切换
function toggleTheme() {
  themeStore.toggleDark();
  const modeText = themeStore.mode === 'auto' ? '自动' : (themeStore.isDark ? '深色' : '浅色');
  ElMessage.success(`已切换到${modeText}模式`);
}

// 获取主题图标
function getThemeIcon() {
  if (themeStore.mode === 'auto') {
    return 'Monitor';
  }
  return themeStore.isDark ? 'Moon' : 'Sunny';
}

// 获取主题提示文字
function getThemeTooltip() {
  if (themeStore.mode === 'auto') {
    return `自动模式 (当前: ${themeStore.isDark ? '深色' : '浅色'})`;
  }
  return themeStore.isDark ? '深色模式' : '浅色模式';
}
</script>

<template>
  <div class="app">
    <el-container>
      <el-header class="app-header">
        <div class="header-content">
          <h1 class="app-title">
            <el-icon>
              <Calendar />
            </el-icon>
            课程表管理系统
          </h1>
          <div class="header-controls">
            <el-radio-group v-model="currentView" size="large">
              <el-radio-button value="schedule">
                <el-icon>
                  <Clock />
                </el-icon>
                课程表
              </el-radio-button>
              <el-radio-button value="settings">
                <el-icon>
                  <Setting />
                </el-icon>
                设置
              </el-radio-button>
            </el-radio-group>

            <el-button
              type="primary"
              size="large"
              @click="toggleTheme"
              :icon="getThemeIcon()"
              circle
              :title="getThemeTooltip()"
            />

            <el-button type="primary" size="large" @click="toggleWidget" :icon="widgetVisible ? 'Hide' : 'View'">
              {{ widgetVisible ? '隐藏' : '显示' }}桌面小组件
            </el-button>

            <el-button type="danger" size="large" @click="quitApp" icon="SwitchButton">
              退出应用
            </el-button>
          </div>
        </div>
      </el-header>

      <el-main class="app-main">
        <el-card class="main-card" shadow="hover">
          <ScheduleDisplay v-if="currentView === 'schedule'" @switch-to-settings="handleSwitchToSettings" />
          <SettingsPanel v-if="currentView === 'settings'" />
        </el-card>
      </el-main>
    </el-container>
  </div>
</template>

<style scoped>
.app {
  height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  transition: background 0.3s ease;
}

/* 深色模式背景 */
.dark .app {
  background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
}

.app-header {
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(10px);
  border-bottom: 1px solid rgba(255, 255, 255, 0.2);
  box-shadow: 0 2px 20px rgba(0, 0, 0, 0.1);
  transition: all 0.3s ease;
}

/* 深色模式头部 */
.dark .app-header {
  background: rgba(26, 26, 46, 0.95);
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  box-shadow: 0 2px 20px rgba(0, 0, 0, 0.3);
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 100%;
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 2rem;
}

.header-controls {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.app-title {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: #2c3e50;
  font-size: 1.5rem;
  font-weight: 600;
  margin: 0;
  transition: color 0.3s ease;
}

/* 深色模式标题 */
.dark .app-title {
  color: #e5eaf3;
}

.app-main {
  padding: 1rem 2rem 2rem 2rem;
  max-width: 1400px;
  margin: 0 auto;
  flex: 1;
  overflow: hidden;
  min-height: 0;
}

.main-card {
  height: 100%;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  display: flex;
  flex-direction: column;
  transition: all 0.3s ease;
}

/* 深色模式卡片 */
.dark .main-card {
  background: rgba(26, 26, 46, 0.95);
  border: 1px solid rgba(255, 255, 255, 0.1);
}

:deep(.el-radio-button__inner) {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.5rem;
  font-weight: 500;
}

:deep(.el-card__body) {
  padding: 0;
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}
</style>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}

#app {
  height: 100vh;
}
</style>