import { reactive, watch } from 'vue';

export type ThemeMode = 'light' | 'dark' | 'auto';

interface ThemeState {
  mode: ThemeMode;
  isDark: boolean;
}

const state = reactive<ThemeState>({
  mode: 'auto',
  isDark: false,
});

class ThemeStore {
  get mode() {
    return state.mode;
  }

  get isDark() {
    return state.isDark;
  }

  // 设置主题模式
  setMode(mode: ThemeMode) {
    state.mode = mode;
    this.updateTheme();
    this.saveToStorage();
  }

  // 切换深色模式
  toggleDark() {
    if (state.mode === 'auto') {
      // 如果当前是自动模式，切换到手动模式
      state.mode = state.isDark ? 'light' : 'dark';
    } else {
      // 如果是手动模式，在 light 和 dark 之间切换
      state.mode = state.mode === 'light' ? 'dark' : 'light';
    }
    this.updateTheme();
    this.saveToStorage();
  }

  // 更新主题
  private updateTheme() {
    let isDark = false;

    if (state.mode === 'auto') {
      // 自动模式：根据系统偏好
      isDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    } else {
      // 手动模式
      isDark = state.mode === 'dark';
    }

    state.isDark = isDark;
    this.applyTheme(isDark);
  }

  // 应用主题到DOM
  private applyTheme(isDark: boolean) {
    const html = document.documentElement;
    
    if (isDark) {
      html.classList.add('dark');
      html.setAttribute('data-theme', 'dark');
    } else {
      html.classList.remove('dark');
      html.setAttribute('data-theme', 'light');
    }

    // 更新Element Plus主题
    if (isDark) {
      html.style.setProperty('--el-color-primary', '#409eff');
      html.style.setProperty('--el-bg-color', '#1a1a1a');
      html.style.setProperty('--el-text-color-primary', '#e5eaf3');
      html.style.setProperty('--el-text-color-regular', '#cfd3dc');
      html.style.setProperty('--el-border-color', '#4c4d4f');
      html.style.setProperty('--el-fill-color-light', '#262727');
    } else {
      html.style.removeProperty('--el-color-primary');
      html.style.removeProperty('--el-bg-color');
      html.style.removeProperty('--el-text-color-primary');
      html.style.removeProperty('--el-text-color-regular');
      html.style.removeProperty('--el-border-color');
      html.style.removeProperty('--el-fill-color-light');
    }
  }

  // 保存到本地存储
  private saveToStorage() {
    try {
      localStorage.setItem('theme-mode', state.mode);
    } catch (error) {
      console.error('Failed to save theme to localStorage:', error);
    }
  }

  // 从本地存储加载
  loadFromStorage() {
    try {
      const stored = localStorage.getItem('theme-mode');
      if (stored && ['light', 'dark', 'auto'].includes(stored)) {
        state.mode = stored as ThemeMode;
      }
    } catch (error) {
      console.error('Failed to load theme from localStorage:', error);
    }
  }

  // 监听系统主题变化
  setupSystemThemeListener() {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    
    const handleChange = () => {
      if (state.mode === 'auto') {
        this.updateTheme();
      }
    };

    // 现代浏览器
    if (mediaQuery.addEventListener) {
      mediaQuery.addEventListener('change', handleChange);
    } else {
      // 兼容旧浏览器
      mediaQuery.addListener(handleChange);
    }

    return () => {
      if (mediaQuery.removeEventListener) {
        mediaQuery.removeEventListener('change', handleChange);
      } else {
        mediaQuery.removeListener(handleChange);
      }
    };
  }

  // 初始化主题
  init() {
    this.loadFromStorage();
    this.updateTheme();
    return this.setupSystemThemeListener();
  }
}

export const themeStore = new ThemeStore();

// 监听主题变化，用于响应式更新
watch(
  () => state.isDark,
  (isDark) => {
    // 可以在这里添加主题变化的副作用
    console.log('Theme changed to:', isDark ? 'dark' : 'light');
  }
);
