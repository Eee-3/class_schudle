/**
 * 课程表相关常量配置
 */

/**
 * 星期名称数组
 */
export const WEEKDAY_NAMES = ['', '周一', '周二', '周三', '周四', '周五', '周六', '周日'];

/**
 * 星期名称数组（用于一周视图）
 */
export const WEEK_DAYS = ['周一', '周二', '周三', '周四', '周五', '周六', '周日'];

/**
 * 时间段配置（8:00-22:00，每小时一个时间段）
 */
export const TIME_SLOTS = [
  '08:00', '09:00', '10:00', '11:00', '12:00', '13:00', '14:00',
  '15:00', '16:00', '17:00', '18:00', '19:00', '20:00', '21:00', '22:00'
];

/**
 * 默认课程颜色列表
 */
export const DEFAULT_COURSE_COLORS = [
  '#3498db', // 蓝色
  '#e74c3c', // 红色
  '#2ecc71', // 绿色
  '#f39c12', // 橙色
  '#9b59b6', // 紫色
  '#1abc9c', // 青色
  '#34495e', // 深灰色
  '#e67e22', // 深橙色
  '#8e44ad', // 深紫色
  '#27ae60', // 深绿色
  '#2980b9', // 深蓝色
  '#c0392b'  // 深红色
];

/**
 * 课程状态颜色配置
 */
export const STATUS_COLORS = {
  CURRENT: '#27ae60',   // 绿色 - 正在进行
  PAST: '#95a5a6',      // 灰色 - 已结束
  UPCOMING: '#f39c12',  // 橙色 - 即将开始
  FUTURE: '#3498db'     // 蓝色 - 未来课程
} as const;

/**
 * 学期配置
 */
export const SEMESTER_CONFIG = {
  // 春季学期：2月1日开始
  SPRING: {
    startMonth: 1, // 2月 (0-based)
    startDay: 1,
    name: '春季学期'
  },
  // 秋季学期：9月1日开始
  AUTUMN: {
    startMonth: 8, // 9月 (0-based)
    startDay: 1,
    name: '秋季学期'
  }
} as const;

/**
 * 时间格式配置
 */
export const TIME_FORMAT = {
  DISPLAY: 'HH:MM',      // 显示格式
  STORAGE: 'HH:MM:SS',   // 存储格式
  LOCALE: 'zh-CN'        // 本地化
} as const;

/**
 * 课程表布局配置
 */
export const LAYOUT_CONFIG = {
  // 主窗口配置
  MAIN_WINDOW: {
    MIN_WIDTH: 1000,
    MIN_HEIGHT: 700,
    DEFAULT_WIDTH: 1200,
    DEFAULT_HEIGHT: 800
  },
  // 桌面小组件配置
  WIDGET: {
    WIDTH: 380,
    HEIGHT: 500,
    MIN_WIDTH: 300,
    MIN_HEIGHT: 400
  },
  // 时间轴配置
  TIME_AXIS: {
    WIDTH: 80,
    SLOT_HEIGHT: 60,
    HEADER_HEIGHT: 80
  }
} as const;

/**
 * 动画配置
 */
export const ANIMATION_CONFIG = {
  TRANSITION_DURATION: '0.3s',
  PULSE_DURATION: '2s',
  HOVER_SCALE: 1.02
} as const;

/**
 * API 配置
 */
export const API_CONFIG = {
  BASE_URL: 'http://localhost:8080',
  ENDPOINTS: {
    SCHEDULE: '/api/v1/schedule',
    COURSES: '/api/v1/courses',
    PUSH: '/api/v1/schedule/push'
  }
} as const;

/**
 * 存储键名配置
 */
export const STORAGE_KEYS = {
  THEME_MODE: 'theme-mode',
  WIDGET_POSITION: 'widget-position',
  BACKEND_URL: 'backend-url',
  SCHEDULE_DATA: 'schedule-data'
} as const;

/**
 * 默认配置值
 */
export const DEFAULT_VALUES = {
  BACKEND_URL: 'http://localhost:8080',
  WIDGET_POSITION: { x: 100, y: 100 },
  COURSE_DURATION: 100, // 默认课程时长（分钟）
  UPCOMING_THRESHOLD: 30 // 即将开始的时间阈值（分钟）
} as const;
