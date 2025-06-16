<template>
  <div class="desktop-widget" :class="{ 'move-mode': moveMode }">
    <!-- 拖拽区域和标题栏 -->
    <div class="widget-header" :data-tauri-drag-region="moveMode ? true : false" @mousedown="onHeaderMouseDown"
      @mouseup="onHeaderMouseUp" @mousemove="onHeaderMouseMove">
      <div class="title">今日课程</div>
      <div class="header-controls">
        <div class="current-time">{{ formatCurrentTime(currentTime) }}</div>
        <div class="control-buttons">
          <button class="control-btn move-btn" @click="toggleMoveMode" :class="{ active: moveMode }">
            {{ moveMode ? '🔒' : '📍' }}
          </button>
          <button class="control-btn settings-btn" @click="openMainApp">
            ⚙️
          </button>
        </div>
      </div>
    </div>

    <!-- 移动模式提示 -->
    <div v-if="moveMode" class="move-hint">
      拖拽标题栏移动位置，再次点击 🔒 锁定
    </div>

    <!-- 课程列表 -->
    <div class="course-list">
      <div v-for="course in todayCourses" :key="course.id" class="course-row" :class="{
        'current-course': isCurrentCourseWrapper(course),
        'past-course': isPastCourseWrapper(course),
        'upcoming-course': isUpcomingCourseWrapper(course)
      }">
        <!-- 左侧时间 -->
        <div class="time-column">
          <div class="time-range">
            <div class="start-time">{{ formatTime(course.start_time) }}</div>
            <div class="time-separator">-</div>
            <div class="end-time">{{ formatTime(course.end_time) }}</div>
          </div>
        </div>

        <!-- 右侧课程信息 -->
        <div class="course-column">
          <div class="course-name">{{ course.name }}</div>
          <div class="course-meta">
            <span v-if="course.teacher" class="teacher">{{ course.teacher }}</span>
            <span v-if="course.location" class="location">{{ course.location }}</span>
          </div>
        </div>

        <!-- 状态指示器 -->
        <div class="status-indicator" :style="{ backgroundColor: getStatusColorWrapper(course) }"></div>
      </div>

      <div v-if="todayCourses.length === 0" class="no-courses">
        <div class="no-courses-icon">📚</div>
        <div class="no-courses-text">今天没有课程</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { scheduleStore } from '../stores/scheduleStore';
import { themeStore } from '../stores/themeStore';
import { invoke } from '@tauri-apps/api/core';
interface Course {
  id: string;
  name: string;
  teacher?: string;
  location?: string;
  weekday: number;
  start_time: string;
  end_time: string;
  weeks: number[];
  color?: string;
}

const currentTime = ref(new Date());
const moveMode = ref(false);
let timeInterval: number;
let desktopLevelInterval: number;

onMounted(async () => {
  scheduleStore.loadSchedule();

  // 初始化主题系统
  themeStore.init();

  timeInterval = setInterval(() => {
    currentTime.value = new Date();
  }, 1000);

  // 恢复小组件位置
  try {
    const [x, y] = await invoke<[number, number]>('restore_widget_position');
    console.log('恢复小组件位置:', x, y);
    // 这里可以设置窗口位置，但 Tauri 会在启动时自动应用
  } catch (error) {
    console.error('恢复小组件位置失败:', error);
  }

  // 设置为桌面小组件模式
  try {
    await invoke('set_desktop_widget_mode');
  } catch (error) {
    console.error('设置桌面小组件模式失败:', error);
  }
});

onUnmounted(() => {
  if (timeInterval) {
    clearInterval(timeInterval);
  }
  if (desktopLevelInterval) {
    clearInterval(desktopLevelInterval);
  }
});

const todayCourses = computed(() => {
  const today = currentTime.value.getDay(); // 0 = Sunday, 1 = Monday, etc.
  const currentWeek = getCurrentWeek();

  return scheduleStore.courses
    .filter(course => {
      // Convert weekday (Monday = 1) to JavaScript day (Monday = 1, Sunday = 0)
      const courseDay = course.weekday === 7 ? 0 : course.weekday;
      return courseDay === today && course.weeks.includes(currentWeek);
    })
    .sort((a, b) => a.start_time.localeCompare(b.start_time));
});

function getCurrentWeek(): number {
  // 智能学期计算：根据当前月份自动选择学期开始时间
  const now = currentTime.value;
  const currentYear = now.getFullYear();

  // 根据月份智能选择学期开始时间
  let semesterStart: Date;
  if (now.getMonth() >= 1 && now.getMonth() <= 6) { // 2-7月
    semesterStart = new Date(currentYear, 1, 1); // 春季学期：2月1日
  } else { // 8-12月或1月
    if (now.getMonth() === 0) { // 1月，使用去年9月
      semesterStart = new Date(currentYear - 1, 8, 1);
    } else { // 8-12月
      semesterStart = new Date(currentYear, 8, 1); // 秋季学期：9月1日
    }
  }

  const diffTime = now.getTime() - semesterStart.getTime();
  const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));
  const week = Math.max(1, Math.ceil(diffDays / 7));

  console.log('小组件 - 学期开始日期:', semesterStart);
  console.log('小组件 - 计算得到的周次:', week);
  return week;
}

function formatTime(timeStr: string): string {
  return timeStr.substring(0, 5); // "HH:MM:SS" -> "HH:MM"
}

function formatCurrentTime(date: Date): string {
  return date.toLocaleTimeString('zh-CN', {
    hour: '2-digit',
    minute: '2-digit',
    hour12: false
  });
}

function isCurrentCourseWrapper(course: Course): boolean {
  const now = currentTime.value;
  const currentTimeStr = `${now.getHours().toString().padStart(2, '0')}:${now.getMinutes().toString().padStart(2, '0')}:00`;
  return currentTimeStr >= course.start_time && currentTimeStr <= course.end_time;
}

function isPastCourseWrapper(course: Course): boolean {
  const now = currentTime.value;
  const currentTimeStr = `${now.getHours().toString().padStart(2, '0')}:${now.getMinutes().toString().padStart(2, '0')}:00`;
  return currentTimeStr > course.end_time;
}

function isUpcomingCourseWrapper(course: Course): boolean {
  const now = currentTime.value;
  const courseStart = course.start_time;

  // 判断是否是接下来30分钟内的课程
  const nowMinutes = now.getHours() * 60 + now.getMinutes();
  const courseStartMinutes = parseInt(courseStart.substring(0, 2)) * 60 + parseInt(courseStart.substring(3, 5));

  return courseStartMinutes > nowMinutes && courseStartMinutes <= nowMinutes + 30;
}

function getStatusColorWrapper(course: Course): string {
  if (isCurrentCourseWrapper(course)) return '#27ae60'; // 绿色 - 正在进行
  if (isPastCourseWrapper(course)) return '#95a5a6'; // 灰色 - 已结束
  if (isUpcomingCourseWrapper(course)) return '#f39c12'; // 橙色 - 即将开始
  return '#3498db'; // 蓝色 - 未来课程
}

// 切换移动模式
async function toggleMoveMode() {
  const newMoveMode = !moveMode.value;
  console.log('切换移动模式:', newMoveMode ? '启用' : '禁用');

  try {
    if (newMoveMode) {
      // 启用拖拽模式
      console.log('拖拽模式已启用');
    } else {
      // 禁用拖拽模式时保存当前位置
      await saveCurrentPosition();
      console.log('拖拽模式已禁用，位置已保存');
    }
    moveMode.value = newMoveMode;

  } catch (error) {
    console.error('切换移动模式失败:', error);
  }
}

// 保存当前窗口位置
async function saveCurrentPosition() {
  try {
    // 获取当前窗口位置
    const [x, y] = await invoke<[number, number]>('get_window_position');
    await invoke('save_widget_position', { x, y });
    console.log('位置已保存:', { x, y });
  } catch (error) {
    console.error('保存位置失败:', error);
  }
}

// 打开主应用
async function openMainApp() {
  console.log('尝试打开主应用...');
  try {
    await invoke('show_main_app');
    console.log('主应用打开成功');
  } catch (error) {
    console.error('打开主应用失败:', error);
  }
}

// 鼠标事件处理 - 用于调试
function onHeaderMouseDown(event: MouseEvent) {
  console.log('Header mousedown:', {
    moveMode: moveMode.value,
    button: event.button,
    target: event.target,
    dragRegion: (event.target as HTMLElement)?.getAttribute('data-tauri-drag-region')
  });
}

function onHeaderMouseUp(event: MouseEvent) {
  console.log('Header mouseup:', {
    moveMode: moveMode.value,
    button: event.button
  });
}

function onHeaderMouseMove(event: MouseEvent) {
  if (moveMode.value) {
    console.log('Header mousemove in move mode:', {
      x: event.clientX,
      y: event.clientY,
      buttons: event.buttons
    });
  }
}
</script>

<style scoped>
/* Windows 桌面小组件样式 - 更高对比度 */
.desktop-widget {
  width: 100%;
  height: 100%;
  background: rgba(255, 255, 255, 0.98);
  backdrop-filter: blur(20px);
  border-radius: 8px;
  padding: 0;
  box-shadow:
    0 4px 20px rgba(0, 0, 0, 0.15),
    0 1px 4px rgba(0, 0, 0, 0.1);
  border: 1px solid rgba(0, 0, 0, 0.1);
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  position: relative;
  overflow: hidden;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  /* 禁用文字选择 */
  user-select: none;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  transition: all 0.3s ease;
}

/* 深色模式桌面小组件 */
.dark .desktop-widget {
  background: rgba(26, 26, 46, 0.98);
  border: 1px solid rgba(255, 255, 255, 0.1);
  box-shadow:
    0 4px 20px rgba(0, 0, 0, 0.4),
    0 1px 4px rgba(0, 0, 0, 0.3);
}



/* 标题栏 - 可拖拽区域 */
.widget-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0;
  padding: 0.75rem;
  border-bottom: 1px solid rgba(0, 0, 0, 0.15);
  position: relative;
  z-index: 1;
  cursor: default;
  border-radius: 8px 8px 0 0;
  background: rgba(240, 242, 245, 0.9);
  flex-shrink: 0;
  transition: all 0.3s ease;
}

.dark .widget-header {
  background: rgba(42, 42, 58, 0.9);
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.header-controls {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.control-buttons {
  display: flex;
  gap: 0.25rem;
}

.control-btn {
  width: 24px;
  height: 24px;
  border: none;
  border-radius: 6px;
  background: rgba(255, 255, 255, 0.6);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  transition: all 0.2s ease;
  backdrop-filter: blur(10px);
}

.control-btn:hover {
  background: rgba(255, 255, 255, 0.8);
  transform: scale(1.1);
}

.control-btn.active {
  background: rgba(52, 152, 219, 0.8);
  color: white;
}

.move-btn.active {
  background: rgba(243, 156, 18, 0.8);
}

/* 移动模式样式 */
.desktop-widget.move-mode {
  border: 2px dashed rgba(243, 156, 18, 0.8);
  box-shadow: 0 0 0 2px rgba(243, 156, 18, 0.2);
}

.desktop-widget.move-mode .widget-header {
  background: rgba(243, 156, 18, 0.2);
  cursor: move;
}

/* 只有在移动模式下，标题栏才显示移动光标 */
.desktop-widget:not(.move-mode) .widget-header {
  cursor: default;
}

.move-hint {
  background: rgba(243, 156, 18, 0.9);
  color: white;
  padding: 0.5rem;
  text-align: center;
  font-size: 0.8rem;
  font-weight: 500;
  border-radius: 0 0 8px 8px;
  animation: moveHintPulse 2s ease-in-out infinite;
}

@keyframes moveHintPulse {

  0%,
  100% {
    background: rgba(243, 156, 18, 0.9);
  }

  50% {
    background: rgba(243, 156, 18, 0.7);
  }
}

.title {
  font-size: 1rem;
  font-weight: 600;
  color: #1a202c;
  opacity: 1;
  transition: color 0.3s ease;
}

.current-time {
  font-size: 1.1rem;
  color: #4a5568;
  font-weight: 500;
  font-variant-numeric: tabular-nums;
  opacity: 1;
  transition: color 0.3s ease;
}

.dark .title {
  color: #e5eaf3;
}

.dark .current-time {
  color: #cfd3dc;
}

/* 课程列表 */
.course-list {
  flex: 1;
  overflow-y: auto;
  scrollbar-width: none;
  position: relative;
  z-index: 1;
  padding: 0.75rem;
  padding-top: 0.5rem;
}

.course-list::-webkit-scrollbar {
  display: none;
}

/* 课程行 */
.course-row {
  display: flex;
  align-items: center;
  padding: 0.6rem;
  margin-bottom: 0.4rem;
  background: rgba(255, 255, 255, 0.9);
  border-radius: 12px;
  border: 1px solid rgba(0, 0, 0, 0.08);
  transition: all 0.2s ease;
  position: relative;
  backdrop-filter: blur(10px);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.course-row:hover {
  background: rgba(255, 255, 255, 1);
  transform: translateY(-1px);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
}

.dark .course-row {
  background: rgba(42, 42, 58, 0.9);
  border: 1px solid rgba(255, 255, 255, 0.08);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
}

.dark .course-row:hover {
  background: rgba(42, 42, 58, 1);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.4);
}

.course-row:last-child {
  margin-bottom: 0;
}

/* 课程状态样式 - 更鲜艳的颜色 */
.course-row.current-course {
  background: rgba(39, 174, 96, 0.25);
  border-color: rgba(39, 174, 96, 0.8);
  border-width: 2px;
  animation: currentPulse 3s ease-in-out infinite;
}

.course-row.past-course {
  opacity: 0.6;
  background: rgba(149, 165, 166, 0.2);
  border-color: rgba(149, 165, 166, 0.5);
}

.course-row.upcoming-course {
  background: rgba(243, 156, 18, 0.25);
  border-color: rgba(243, 156, 18, 0.8);
  border-width: 2px;
}

@keyframes currentPulse {

  0%,
  100% {
    box-shadow: 0 2px 8px rgba(39, 174, 96, 0.4);
    border-color: rgba(39, 174, 96, 0.8);
  }

  50% {
    box-shadow: 0 4px 16px rgba(39, 174, 96, 0.6);
    border-color: rgba(39, 174, 96, 1);
  }
}

/* 时间列 */
.time-column {
  min-width: 70px;
  margin-right: 0.6rem;
}

.time-range {
  display: flex;
  flex-direction: column;
  align-items: center;
  font-variant-numeric: tabular-nums;
}

.start-time {
  font-size: 1.1rem;
  font-weight: 600;
  color: #1a202c;
  line-height: 1;
  transition: color 0.3s ease;
}

.time-separator {
  font-size: 0.6rem;
  color: #718096;
  margin: 0.1rem 0;
  transition: color 0.3s ease;
}

.end-time {
  font-size: 0.9rem;
  color: #4a5568;
  line-height: 1;
  transition: color 0.3s ease;
}

.dark .start-time {
  color: #e5eaf3;
}

.dark .time-separator {
  color: #a0aec0;
}

.dark .end-time {
  color: #cfd3dc;
}

/* 课程信息列 */
.course-column {
  flex: 1;
  min-width: 0;
}

.course-name {
  font-size: 1.1rem;
  font-weight: 600;
  color: #1a202c;
  margin-bottom: 0.2rem;
  line-height: 1.2;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  transition: color 0.3s ease;
}

.course-meta {
  display: flex;
  flex-direction: column;
  gap: 0.05rem;
  font-size: 0.7rem;
  color: #4a5568;
  transition: color 0.3s ease;
}

.dark .course-name {
  color: #e5eaf3;
}

.dark .course-meta {
  color: #cfd3dc;
}

.teacher,
.location {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.teacher::before {
  content: "👨‍🏫 ";
  font-size: 0.6rem;
}

.location::before {
  content: "📍 ";
  font-size: 0.6rem;
}

/* 状态指示器 */
.status-indicator {
  width: 3px;
  height: 80%;
  border-radius: 1.5px;
  position: absolute;
  right: 0.4rem;
  top: 50%;
  transform: translateY(-50%);
}

/* 无课程状态 */
.no-courses {
  text-align: center;
  padding: 1.5rem 1rem;
  color: #7f8c8d;
  opacity: 0.8;
  transition: color 0.3s ease;
}

.no-courses-icon {
  font-size: 1.5rem;
  margin-bottom: 0.4rem;
  opacity: 0.6;
}

.no-courses-text {
  font-size: 0.8rem;
  font-weight: 500;
}

.dark .no-courses {
  color: #cfd3dc;
}
</style>
