import { reactive } from 'vue';
import { apiService, type Course, type CreateCourseRequest, type UpdateCourseRequest } from '../services/apiService';

interface ScheduleState {
  courses: Course[];
  loading: boolean;
  error: string | null;
}

const state = reactive<ScheduleState>({
  courses: [],
  loading: false,
  error: null,
});

class ScheduleStore {
  get courses() {
    return state.courses;
  }

  get loading() {
    return state.loading;
  }

  get error() {
    return state.error;
  }

  // 从本地存储加载课程表
  loadFromLocalStorage() {
    try {
      const stored = localStorage.getItem('schedule');
      if (stored) {
        const data = JSON.parse(stored);
        state.courses = data.courses || [];
      }
    } catch (error) {
      console.error('Failed to load from localStorage:', error);
    }
  }

  // 保存到本地存储
  saveToLocalStorage() {
    try {
      const data = {
        courses: state.courses,
        lastUpdated: new Date().toISOString(),
      };
      localStorage.setItem('schedule', JSON.stringify(data));
    } catch (error) {
      console.error('Failed to save to localStorage:', error);
    }
  }

  // 从后端加载课程表
  async loadSchedule() {
    state.loading = true;
    state.error = null;
    
    try {
      const schedule = await apiService.getSchedule();
      state.courses = schedule.courses;
      this.saveToLocalStorage();
    } catch (error) {
      state.error = (error as Error).message;
      // 如果后端请求失败，尝试从本地存储加载
      this.loadFromLocalStorage();
      throw error;
    } finally {
      state.loading = false;
    }
  }

  // 创建课程
  async createCourse(courseData: CreateCourseRequest) {
    state.loading = true;
    state.error = null;

    try {
      const newCourse = await apiService.createCourse(courseData);
      state.courses.push(newCourse);
      this.saveToLocalStorage();
      return newCourse;
    } catch (error) {
      state.error = (error as Error).message;
      // 如果后端请求失败，添加到本地存储
      const localCourse: Course = {
        id: Date.now().toString(),
        ...courseData,
      };
      state.courses.push(localCourse);
      this.saveToLocalStorage();
      return localCourse;
    } finally {
      state.loading = false;
    }
  }

  // 更新课程
  async updateCourse(id: string, courseData: UpdateCourseRequest) {
    state.loading = true;
    state.error = null;

    try {
      const updatedCourse = await apiService.updateCourse(id, courseData);
      const index = state.courses.findIndex(c => c.id === id);
      if (index !== -1) {
        state.courses[index] = updatedCourse;
      }
      this.saveToLocalStorage();
      return updatedCourse;
    } catch (error) {
      state.error = (error as Error).message;
      // 如果后端请求失败，更新本地存储
      const index = state.courses.findIndex(c => c.id === id);
      if (index !== -1) {
        state.courses[index] = { ...state.courses[index], ...courseData };
        this.saveToLocalStorage();
      }
      throw error;
    } finally {
      state.loading = false;
    }
  }

  // 删除课程
  async deleteCourse(id: string) {
    state.loading = true;
    state.error = null;

    try {
      await apiService.deleteCourse(id);
      state.courses = state.courses.filter(c => c.id !== id);
      this.saveToLocalStorage();
    } catch (error) {
      state.error = (error as Error).message;
      // 如果后端请求失败，从本地存储删除
      state.courses = state.courses.filter(c => c.id !== id);
      this.saveToLocalStorage();
      throw error;
    } finally {
      state.loading = false;
    }
  }

  // 清空所有课程
  clearCourses() {
    state.courses = [];
    this.saveToLocalStorage();
  }

  // 添加课程到本地（不调用后端）
  addCourseLocal(course: Course) {
    state.courses.push(course);
    this.saveToLocalStorage();
  }

  // 批量导入课程
  async importCourses(courses: CreateCourseRequest[], replace = false) {
    state.loading = true;
    state.error = null;

    try {
      const result = await apiService.pushSchedule({ courses, replace });
      state.courses = result.courses;
      this.saveToLocalStorage();
      return result;
    } catch (error) {
      state.error = (error as Error).message;
      // 如果后端请求失败，更新本地存储
      if (replace) {
        state.courses = [];
      }
      courses.forEach(courseData => {
        const localCourse: Course = {
          id: Date.now().toString() + Math.random().toString(36).substr(2, 9),
          ...courseData,
        };
        state.courses.push(localCourse);
      });
      this.saveToLocalStorage();
      throw error;
    } finally {
      state.loading = false;
    }
  }
}

export const scheduleStore = new ScheduleStore();

// 初始化时从本地存储加载
scheduleStore.loadFromLocalStorage();
