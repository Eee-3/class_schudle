/**
 * 课程表相关类型定义
 */

export interface Course {
  id: string;
  name: string;
  teacher?: string;
  location?: string;
  weekday: number; // 1=周一, 2=周二, ..., 7=周日
  start_time: string; // "HH:MM:SS" 格式
  end_time: string; // "HH:MM:SS" 格式
  weeks: number[]; // 第几周的数组
  color?: string; // 课程颜色，十六进制格式如 "#3498db"
}

export interface Schedule {
  courses: Course[];
}

export interface CreateCourseRequest {
  name: string;
  teacher?: string;
  location?: string;
  weekday: number;
  start_time: string;
  end_time: string;
  weeks: number[];
  color?: string;
}

export interface UpdateCourseRequest {
  name?: string;
  teacher?: string;
  location?: string;
  weekday?: number;
  start_time?: string;
  end_time?: string;
  weeks?: number[];
  color?: string;
}

/**
 * 课程状态枚举
 */
export enum CourseStatus {
  PAST = 'past',        // 已结束
  CURRENT = 'current',  // 正在进行
  UPCOMING = 'upcoming', // 即将开始
  FUTURE = 'future'     // 未来课程
}

/**
 * 星期枚举
 */
export enum Weekday {
  MONDAY = 1,
  TUESDAY = 2,
  WEDNESDAY = 3,
  THURSDAY = 4,
  FRIDAY = 5,
  SATURDAY = 6,
  SUNDAY = 7
}

/**
 * 主题模式类型
 */
export type ThemeMode = 'light' | 'dark' | 'auto';

/**
 * 时间段配置
 */
export interface TimeSlotConfig {
  start: string; // "HH:MM"
  end: string;   // "HH:MM"
  label: string; // 显示标签
}

/**
 * 学期配置
 */
export interface SemesterConfig {
  name: string;           // 学期名称，如 "2024春季学期"
  startDate: Date;        // 学期开始日期
  endDate: Date;          // 学期结束日期
  totalWeeks: number;     // 总周数
}

/**
 * 课程表显示配置
 */
export interface ScheduleDisplayConfig {
  showWeekend: boolean;   // 是否显示周末
  timeSlots: TimeSlotConfig[]; // 时间段配置
  defaultColors: string[]; // 默认课程颜色列表
}
