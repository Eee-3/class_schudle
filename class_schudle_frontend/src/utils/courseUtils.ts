/**
 * 课程表相关工具函数
 * 统一管理时间计算、课程状态判断、格式化等功能
 */

import type { Course } from '../types/course';
import { CourseStatus } from '../types/course';
import { WEEKDAY_NAMES, STATUS_COLORS, SEMESTER_CONFIG, DEFAULT_VALUES } from '../constants/schedule';

/**
 * 智能计算当前学期周次
 * 根据当前月份自动选择学期开始时间
 */
export function getCurrentWeek(currentTime: Date = new Date()): number {
  const now = currentTime;
  const currentYear = now.getFullYear();

  // 根据月份智能选择学期开始时间
  let semesterStart: Date;
  if (now.getMonth() >= 1 && now.getMonth() <= 6) { // 2-7月
    semesterStart = new Date(currentYear, SEMESTER_CONFIG.SPRING.startMonth, SEMESTER_CONFIG.SPRING.startDay);
  } else { // 8-12月或1月
    if (now.getMonth() === 0) { // 1月，使用去年9月
      semesterStart = new Date(currentYear - 1, SEMESTER_CONFIG.AUTUMN.startMonth, SEMESTER_CONFIG.AUTUMN.startDay);
    } else { // 8-12月
      semesterStart = new Date(currentYear, SEMESTER_CONFIG.AUTUMN.startMonth, SEMESTER_CONFIG.AUTUMN.startDay);
    }
  }

  const diffTime = now.getTime() - semesterStart.getTime();
  const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));
  const week = Math.max(1, Math.ceil(diffDays / 7));

  console.log('学期开始日期:', semesterStart);
  console.log('计算得到的周次:', week);
  return week;
}

/**
 * 格式化时间字符串
 * @param timeStr "HH:MM:SS" 格式的时间字符串
 * @returns "HH:MM" 格式的时间字符串
 */
export function formatTime(timeStr: string): string {
  return timeStr.substring(0, 5); // "HH:MM:SS" -> "HH:MM"
}

/**
 * 格式化当前时间为中文格式
 * @param date Date 对象
 * @returns "HH:MM" 格式的时间字符串
 */
export function formatCurrentTime(date: Date): string {
  return date.toLocaleTimeString('zh-CN', {
    hour: '2-digit',
    minute: '2-digit',
    hour12: false
  });
}

/**
 * 获取星期几的中文名称
 * @param weekday 1=周一, 2=周二, ..., 7=周日
 * @returns 中文星期名称
 */
export function getWeekdayName(weekday: number): string {
  return WEEKDAY_NAMES[weekday] || '';
}

/**
 * 判断指定日期索引是否为今天
 * @param dayIndex 0=周一, 1=周二, ..., 6=周日
 * @param currentTime 当前时间，默认为 new Date()
 * @returns 是否为今天
 */
export function isToday(dayIndex: number, currentTime: Date = new Date()): boolean {
  const today = currentTime.getDay();
  // 转换：周一=1 对应 dayIndex=0
  const todayIndex = today === 0 ? 6 : today - 1;
  return dayIndex === todayIndex;
}

/**
 * 格式化日期显示
 * @param dayIndex 0=周一, 1=周二, ..., 6=周日
 * @param currentTime 当前时间，默认为 new Date()
 * @returns "M/D" 格式的日期字符串
 */
export function formatDate(dayIndex: number, currentTime: Date = new Date()): string {
  const today = new Date(currentTime);
  const currentDayIndex = today.getDay() === 0 ? 6 : today.getDay() - 1;
  const diff = dayIndex - currentDayIndex;
  const targetDate = new Date(today.getTime() + diff * 24 * 60 * 60 * 1000);
  return `${targetDate.getMonth() + 1}/${targetDate.getDate()}`;
}

/**
 * 判断指定时间段是否为当前时间段
 * @param timeSlot "HH:MM" 格式的时间字符串
 * @param currentTime 当前时间，默认为 new Date()
 * @returns 是否为当前时间段
 */
export function isCurrentTimeSlot(timeSlot: string, currentTime: Date = new Date()): boolean {
  const currentHour = currentTime.getHours();
  const slotHour = parseInt(timeSlot.split(':')[0]);
  return currentHour === slotHour;
}

/**
 * 判断课程是否正在进行
 * @param course 课程对象
 * @param currentTime 当前时间，默认为 new Date()
 * @returns 是否正在进行
 */
export function isCurrentCourse(course: Course, currentTime: Date = new Date()): boolean {
  // 检查是否为今天
  const dayIndex = course.weekday === 7 ? 6 : course.weekday - 1;
  if (!isToday(dayIndex, currentTime)) return false;

  const now = currentTime;
  const currentTimeStr = `${now.getHours().toString().padStart(2, '0')}:${now.getMinutes().toString().padStart(2, '0')}:00`;
  return currentTimeStr >= course.start_time && currentTimeStr <= course.end_time;
}

/**
 * 判断课程是否已经结束
 * @param course 课程对象
 * @param currentTime 当前时间，默认为 new Date()
 * @returns 是否已经结束
 */
export function isPastCourse(course: Course, currentTime: Date = new Date()): boolean {
  const dayIndex = course.weekday === 7 ? 6 : course.weekday - 1;
  const today = currentTime.getDay() === 0 ? 6 : currentTime.getDay() - 1;

  if (dayIndex < today) return true;
  if (dayIndex > today) return false;

  // 同一天，检查时间
  const now = currentTime;
  const currentTimeStr = `${now.getHours().toString().padStart(2, '0')}:${now.getMinutes().toString().padStart(2, '0')}:00`;
  return currentTimeStr > course.end_time;
}

/**
 * 判断课程是否即将开始（30分钟内）
 * @param course 课程对象
 * @param currentTime 当前时间，默认为 new Date()
 * @returns 是否即将开始
 */
export function isUpcomingCourse(course: Course, currentTime: Date = new Date()): boolean {
  // 检查是否为今天
  const dayIndex = course.weekday === 7 ? 6 : course.weekday - 1;
  if (!isToday(dayIndex, currentTime)) return false;

  const now = currentTime;
  const courseStart = course.start_time;

  // 判断是否是接下来指定时间内的课程
  const nowMinutes = now.getHours() * 60 + now.getMinutes();
  const courseStartMinutes = parseInt(courseStart.substring(0, 2)) * 60 + parseInt(courseStart.substring(3, 5));

  return courseStartMinutes > nowMinutes && courseStartMinutes <= nowMinutes + DEFAULT_VALUES.UPCOMING_THRESHOLD;
}

/**
 * 获取课程状态
 * @param course 课程对象
 * @param currentTime 当前时间，默认为 new Date()
 * @returns 课程状态
 */
export function getCourseStatus(course: Course, currentTime: Date = new Date()): CourseStatus {
  if (isCurrentCourse(course, currentTime)) return CourseStatus.CURRENT;
  if (isPastCourse(course, currentTime)) return CourseStatus.PAST;
  if (isUpcomingCourse(course, currentTime)) return CourseStatus.UPCOMING;
  return CourseStatus.FUTURE;
}

/**
 * 获取课程状态对应的颜色
 * @param course 课程对象
 * @param currentTime 当前时间，默认为 new Date()
 * @returns 颜色值
 */
export function getStatusColor(course: Course, currentTime: Date = new Date()): string {
  const status = getCourseStatus(course, currentTime);
  switch (status) {
    case CourseStatus.CURRENT:
      return STATUS_COLORS.CURRENT;
    case CourseStatus.PAST:
      return STATUS_COLORS.PAST;
    case CourseStatus.UPCOMING:
      return STATUS_COLORS.UPCOMING;
    case CourseStatus.FUTURE:
    default:
      return STATUS_COLORS.FUTURE;
  }
}

/**
 * 计算课程持续时间（分钟）
 * @param course 课程对象
 * @returns 持续时间（分钟）
 */
export function getCourseDuration(course: Course): number {
  const startTime = course.start_time.split(':');
  const endTime = course.end_time.split(':');
  const startMinutes = parseInt(startTime[0]) * 60 + parseInt(startTime[1]);
  const endMinutes = parseInt(endTime[0]) * 60 + parseInt(endTime[1]);
  const durationMinutes = endMinutes - startMinutes;

  // 每分钟对应1像素，最小高度60px
  return Math.max(durationMinutes, 60);
}

/**
 * 获取指定日期和时间的课程列表
 * @param courses 所有课程列表
 * @param dayIndex 0=周一, 1=周二, ..., 6=周日
 * @param timeSlot "HH:MM" 格式的时间字符串
 * @returns 符合条件的课程列表
 */
export function getCoursesForDayAndTime(courses: Course[], dayIndex: number, timeSlot: string): Course[] {
  // dayIndex: 0=周一, 1=周二, ..., 6=周日
  // 转换为课程表的 weekday: 1=周一, 2=周二, ..., 7=周日
  const weekday = dayIndex === 6 ? 7 : dayIndex + 1;

  return courses.filter(course => {
    if (course.weekday !== weekday) return false;

    // 检查课程是否在这个时间段开始
    const courseStartHour = parseInt(course.start_time.split(':')[0]);
    const slotHour = parseInt(timeSlot.split(':')[0]);

    return courseStartHour === slotHour;
  });
}

/**
 * 过滤当前周的课程
 * @param courses 所有课程列表
 * @param currentTime 当前时间，默认为 new Date()
 * @returns 当前周的课程列表
 */
export function getCurrentWeekCourses(courses: Course[], currentTime: Date = new Date()): Course[] {
  const currentWeek = getCurrentWeek(currentTime);
  return courses.filter(course => course.weeks.includes(currentWeek));
}

/**
 * 过滤今天的课程
 * @param courses 所有课程列表
 * @param currentTime 当前时间，默认为 new Date()
 * @returns 今天的课程列表，按时间排序
 */
export function getTodayCourses(courses: Course[], currentTime: Date = new Date()): Course[] {
  const today = currentTime.getDay(); // 0 = Sunday, 1 = Monday, etc.
  const currentWeek = getCurrentWeek(currentTime);

  return courses
    .filter(course => {
      // Convert course weekday (1=Monday, 7=Sunday) to JavaScript day (0=Sunday, 1=Monday)
      const courseDay = course.weekday === 7 ? 0 : course.weekday;
      return courseDay === today && course.weeks.includes(currentWeek);
    })
    .sort((a, b) => a.start_time.localeCompare(b.start_time));
}
