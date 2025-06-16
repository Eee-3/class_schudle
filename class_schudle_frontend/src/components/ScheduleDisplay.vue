<template>
  <div class="schedule-display">
    <div class="schedule-header">
      <h2>本周课程表</h2>
      <div class="week-info">
        <div class="current-time">{{ formatCurrentTime(currentTime) }}</div>
        <div class="week-number">第 {{ currentWeek }} 周</div>
      </div>
    </div>

    <!-- 空状态显示 -->
    <div v-if="scheduleStore.courses.length === 0" class="empty-state">
      <div class="empty-icon">📅</div>
      <h3>暂无课程数据</h3>
      <p>请前往设置页面添加课程或导入课程表</p>
      <el-button type="primary" @click="$emit('switch-to-settings')">
        前往设置
      </el-button>
    </div>

    <!-- 本周无课程显示 -->
    <div v-else-if="weekCourses.length === 0" class="empty-state">
      <div class="empty-icon">🎉</div>
      <h3>本周无课程安排</h3>
      <p>第 {{ currentWeek }} 周没有课程，好好休息吧！</p>
    </div>

    <!-- 正常课表显示 -->
    <div v-else class="weekly-schedule">
      <!-- 固定头部 -->
      <div class="schedule-header-row">
        <div class="time-header">时间</div>
        <div v-for="(dayName, dayIndex) in weekDays" :key="dayIndex" class="day-header"
          :class="{ 'today': isTodayWrapper(dayIndex) }">
          <div class="day-name">{{ dayName }}</div>
          <div class="day-date">{{ formatDateWrapper(dayIndex) }}</div>
        </div>
      </div>

      <!-- 可滚动内容区域 -->
      <div class="schedule-content">
        <div class="schedule-grid">
          <!-- 时间轴 -->
          <div class="time-column">
            <div v-for="timeSlot in timeSlots" :key="timeSlot" class="time-slot"
              :class="{ 'current-time-slot': isCurrentTimeSlotWrapper(timeSlot) }">
              {{ timeSlot }}
            </div>
          </div>

          <!-- 每天的课程列 -->
          <div v-for="(_, dayIndex) in weekDays" :key="dayIndex" class="day-content-column"
            :class="{ 'today': isTodayWrapper(dayIndex) }">
            <div v-for="timeSlot in timeSlots" :key="timeSlot" class="course-slot">
              <div v-for="course in getCoursesForDayAndTimeWrapper(dayIndex, timeSlot)" :key="course.id" class="course-card"
                :class="{
                  'current-course': isCurrentCourseWrapper(course),
                  'past-course': isPastCourseWrapper(course)
                }" :style="{
                  backgroundColor: course.color || '#3498db',
                  height: getCourseDurationWrapper(course) + 'px'
                }">
                <div class="course-name">{{ course.name }}</div>
                <div class="course-time">{{ formatTime(course.start_time) }}-{{ formatTime(course.end_time) }}</div>
                <div class="course-details">
                  <div v-if="course.teacher" class="teacher">{{ course.teacher }}</div>
                  <div v-if="course.location" class="location">{{ course.location }}</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { scheduleStore } from '../stores/scheduleStore';

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
let timeInterval: number;

// 一周的天数
const weekDays = ['周一', '周二', '周三', '周四', '周五', '周六', '周日'];

// 时间段（从早8点到晚10点，每小时一个时间段）
const timeSlots = [
  '08:00', '09:00', '10:00', '11:00', '12:00', '13:00', '14:00',
  '15:00', '16:00', '17:00', '18:00', '19:00', '20:00', '21:00', '22:00'
];

onMounted(() => {
  scheduleStore.loadSchedule();
  timeInterval = setInterval(() => {
    currentTime.value = new Date();
  }, 1000);
});

onUnmounted(() => {
  if (timeInterval) {
    clearInterval(timeInterval);
  }
});

const currentWeek = computed(() => getCurrentWeek());

// 获取本周所有课程
const weekCourses = computed(() => {
  const week = currentWeek.value;
  console.log('当前周次:', week);
  console.log('所有课程:', scheduleStore.courses);
  const filtered = scheduleStore.courses.filter(course => course.weeks.includes(week));
  console.log('本周课程:', filtered);
  return filtered;
});

function getCurrentWeek(): number {
  // 修改：假设学期从当前年份的2月1日开始（适应春季学期）
  const now = currentTime.value;
  const currentYear = now.getFullYear();

  // 如果当前月份在2-7月，使用当前年份的2月1日作为学期开始
  // 如果当前月份在8-12月或1月，使用当前年份的9月1日作为学期开始
  let semesterStart: Date;
  if (now.getMonth() >= 1 && now.getMonth() <= 6) { // 2-7月
    semesterStart = new Date(currentYear, 1, 1); // February 1st
  } else { // 8-12月或1月
    if (now.getMonth() === 0) { // 1月，使用去年9月
      semesterStart = new Date(currentYear - 1, 8, 1);
    } else { // 8-12月
      semesterStart = new Date(currentYear, 8, 1);
    }
  }

  const diffTime = now.getTime() - semesterStart.getTime();
  const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));
  const week = Math.max(1, Math.ceil(diffDays / 7));
  console.log('学期开始日期:', semesterStart);
  console.log('计算得到的周次:', week);
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

function isTodayWrapper(dayIndex: number): boolean {
  const today = currentTime.value.getDay();
  // 转换：周一=1 对应 dayIndex=0
  const todayIndex = today === 0 ? 6 : today - 1;
  return dayIndex === todayIndex;
}

function formatDateWrapper(dayIndex: number): string {
  const today = new Date(currentTime.value);
  const currentDayIndex = today.getDay() === 0 ? 6 : today.getDay() - 1;
  const diff = dayIndex - currentDayIndex;
  const targetDate = new Date(today.getTime() + diff * 24 * 60 * 60 * 1000);
  return `${targetDate.getMonth() + 1}/${targetDate.getDate()}`;
}

function isCurrentTimeSlotWrapper(timeSlot: string): boolean {
  const now = currentTime.value;
  const currentHour = now.getHours();
  const slotHour = parseInt(timeSlot.split(':')[0]);
  return currentHour === slotHour;
}

function getCoursesForDayAndTimeWrapper(dayIndex: number, timeSlot: string): Course[] {
  // dayIndex: 0=周一, 1=周二, ..., 6=周日
  // 转换为课程表的 weekday: 1=周一, 2=周二, ..., 7=周日
  const weekday = dayIndex === 6 ? 7 : dayIndex + 1;

  return weekCourses.value.filter(course => {
    if (course.weekday !== weekday) return false;

    // 检查课程是否在这个时间段开始
    const courseStartHour = parseInt(course.start_time.split(':')[0]);
    const slotHour = parseInt(timeSlot.split(':')[0]);

    return courseStartHour === slotHour;
  });
}

function getCourseDurationWrapper(course: Course): number {
  const startTime = course.start_time.split(':');
  const endTime = course.end_time.split(':');
  const startMinutes = parseInt(startTime[0]) * 60 + parseInt(startTime[1]);
  const endMinutes = parseInt(endTime[0]) * 60 + parseInt(endTime[1]);
  const durationMinutes = endMinutes - startMinutes;

  // 每分钟对应1像素，最小高度60px
  return Math.max(durationMinutes, 60);
}

function isCurrentCourseWrapper(course: Course): boolean {
  if (!isTodayWrapper(course.weekday === 7 ? 6 : course.weekday - 1)) return false;

  const now = currentTime.value;
  const currentTimeStr = `${now.getHours().toString().padStart(2, '0')}:${now.getMinutes().toString().padStart(2, '0')}:00`;
  return currentTimeStr >= course.start_time && currentTimeStr <= course.end_time;
}

function isPastCourseWrapper(course: Course): boolean {
  const dayIndex = course.weekday === 7 ? 6 : course.weekday - 1;
  const today = currentTime.value.getDay() === 0 ? 6 : currentTime.value.getDay() - 1;

  if (dayIndex < today) return true;
  if (dayIndex > today) return false;

  // 同一天，检查时间
  const now = currentTime.value;
  const currentTimeStr = `${now.getHours().toString().padStart(2, '0')}:${now.getMinutes().toString().padStart(2, '0')}:00`;
  return currentTimeStr > course.end_time;
}
</script>

<style scoped>
.schedule-display {
  height: 100%;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(10px);
  padding: 1.5rem;
  border-radius: 12px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  min-height: 0;
  transition: all 0.3s ease;
}

/* 深色模式 */
.dark .schedule-display {
  background: rgba(26, 26, 46, 0.95);
}

.schedule-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
  padding-bottom: 1rem;
  border-bottom: 2px solid #ecf0f1;
  flex-shrink: 0;
}

.schedule-header h2 {
  color: #2c3e50;
  font-size: 1.5rem;
  font-weight: 600;
  margin: 0;
  transition: color 0.3s ease;
}

.dark .schedule-header h2 {
  color: #e5eaf3;
}

.week-info {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 0.25rem;
}

.current-time {
  font-size: 1.1rem;
  color: #2c3e50;
  font-weight: 600;
  font-variant-numeric: tabular-nums;
  transition: color 0.3s ease;
}

.week-number {
  font-size: 0.9rem;
  color: #7f8c8d;
  font-weight: 500;
  transition: color 0.3s ease;
}

.dark .current-time {
  color: #e5eaf3;
}

.dark .week-number {
  color: #cfd3dc;
}

/* 空状态样式 */
.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
  padding: 3rem;
  color: #7f8c8d;
  transition: color 0.3s ease;
}

.empty-icon {
  font-size: 4rem;
  margin-bottom: 1rem;
}

.empty-state h3 {
  color: #2c3e50;
  margin-bottom: 0.5rem;
  font-size: 1.2rem;
  transition: color 0.3s ease;
}

.empty-state p {
  margin-bottom: 1.5rem;
  font-size: 0.9rem;
}

/* 深色模式空状态 */
.dark .empty-state {
  color: #cfd3dc;
}

.dark .empty-state h3 {
  color: #e5eaf3;
}

/* 一周视图布局 - 完全适应容器 */
.weekly-schedule {
  display: flex;
  flex-direction: column;
  flex: 1;
  min-height: 500px;
  max-height: calc(100vh - 200px);
  border: 1px solid #e1e8ed;
  border-radius: 8px;
  overflow: hidden;
  background: white;
  transition: all 0.3s ease;
}

.dark .weekly-schedule {
  border: 1px solid #4c4d4f;
  background: #1a1a1a;
}

/* 固定头部 */
.schedule-header-row {
  display: flex;
  background: #f8f9fa;
  border-bottom: 2px solid #e1e8ed;
  flex-shrink: 0;
  transition: all 0.3s ease;
}

.dark .schedule-header-row {
  background: #262727;
  border-bottom: 2px solid #4c4d4f;
}

.time-header {
  width: 80px;
  height: 80px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  color: #2c3e50;
  background: #ecf0f1;
  border-right: 1px solid #e1e8ed;
  font-size: 0.9rem;
  flex-shrink: 0;
  transition: all 0.3s ease;
}

.dark .time-header {
  color: #e5eaf3;
  background: #2a2a2a;
  border-right: 1px solid #4c4d4f;
}

.day-header {
  flex: 1;
  height: 80px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  background: #f8f9fa;
  border-right: 1px solid #e1e8ed;
  gap: 0.25rem;
  min-width: 120px;
  transition: all 0.3s ease;
}

.day-header:last-child {
  border-right: none;
}

.day-header.today {
  background: rgba(52, 152, 219, 0.1);
}

.day-name {
  font-weight: 600;
  color: #2c3e50;
  font-size: 0.9rem;
  transition: color 0.3s ease;
}

.day-date {
  font-size: 0.8rem;
  color: #7f8c8d;
  transition: color 0.3s ease;
}

.day-header.today .day-name {
  color: #3498db;
}

/* 深色模式天头部 */
.dark .day-header {
  background: #262727;
  border-right: 1px solid #4c4d4f;
}

.dark .day-header.today {
  background: rgba(52, 152, 219, 0.2);
}

.dark .day-name {
  color: #e5eaf3;
}

.dark .day-date {
  color: #cfd3dc;
}

/* 可滚动内容区域 */
.schedule-content {
  flex: 1;
  overflow-y: auto;
  min-height: 0;
}

.schedule-grid {
  display: flex;
  min-height: 100%;
}

/* 时间列 */
.time-column {
  width: 80px;
  background: #f8f9fa;
  border-right: 1px solid #e1e8ed;
  flex-shrink: 0;
  transition: all 0.3s ease;
}

.time-slot {
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-bottom: 1px solid #e1e8ed;
  font-size: 0.8rem;
  color: #7f8c8d;
  font-variant-numeric: tabular-nums;
  transition: all 0.3s ease;
}

.time-slot.current-time-slot {
  background: rgba(52, 152, 219, 0.1);
  color: #3498db;
  font-weight: 600;
}

/* 深色模式时间列 */
.dark .time-column {
  background: #262727;
  border-right: 1px solid #4c4d4f;
}

.dark .time-slot {
  border-bottom: 1px solid #4c4d4f;
  color: #cfd3dc;
}

.dark .time-slot.current-time-slot {
  background: rgba(52, 152, 219, 0.2);
  color: #409eff;
}

/* 天内容列 */
.day-content-column {
  flex: 1;
  border-right: 1px solid #e1e8ed;
  min-width: 120px;
  transition: all 0.3s ease;
}

.day-content-column:last-child {
  border-right: none;
}

.day-content-column.today {
  background: rgba(52, 152, 219, 0.02);
}

/* 深色模式天内容列 */
.dark .day-content-column {
  border-right: 1px solid #4c4d4f;
}

.dark .day-content-column.today {
  background: rgba(52, 152, 219, 0.05);
}

.course-slot {
  height: 60px;
  border-bottom: 1px solid #e1e8ed;
  position: relative;
  padding: 2px;
  transition: border-color 0.3s ease;
}

.dark .course-slot {
  border-bottom: 1px solid #4c4d4f;
}

/* 课程卡片 */
.course-card {
  position: absolute;
  left: 2px;
  right: 2px;
  top: 2px;
  border-radius: 4px;
  padding: 0.5rem;
  color: white;
  font-size: 0.75rem;
  line-height: 1.2;
  overflow: hidden;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
}

.course-card:hover {
  transform: scale(1.02);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
  z-index: 10;
}

.course-card.current-course {
  animation: currentPulse 2s ease-in-out infinite;
  box-shadow: 0 0 0 2px rgba(255, 255, 255, 0.8);
}

.course-card.past-course {
  opacity: 0.6;
  filter: grayscale(0.3);
}

@keyframes currentPulse {

  0%,
  100% {
    box-shadow: 0 0 0 2px rgba(255, 255, 255, 0.8);
  }

  50% {
    box-shadow: 0 0 0 4px rgba(255, 255, 255, 0.6);
  }
}

.course-name {
  font-weight: 600;
  margin-bottom: 0.25rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.course-time {
  font-size: 0.7rem;
  opacity: 0.9;
  margin-bottom: 0.25rem;
}

.course-details {
  font-size: 0.65rem;
  opacity: 0.8;
}

.course-details .teacher,
.course-details .location {
  display: block;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* 响应式布局 */
@media (max-height: 700px) {
  .weekly-schedule {
    min-height: 400px;
    max-height: calc(100vh - 150px);
  }

  .time-slot {
    height: 50px !important;
  }

  .course-slot {
    height: 50px !important;
  }

  .day-header, .time-header {
    height: 60px !important;
  }
}

@media (max-height: 600px) {
  .weekly-schedule {
    min-height: 350px;
    max-height: calc(100vh - 120px);
  }

  .time-slot {
    height: 40px !important;
  }

  .course-slot {
    height: 40px !important;
  }

  .day-header, .time-header {
    height: 50px !important;
  }
}
</style>
