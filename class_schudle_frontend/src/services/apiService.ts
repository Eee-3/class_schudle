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

interface CreateCourseRequest {
  name: string;
  teacher?: string;
  location?: string;
  weekday: number;
  start_time: string;
  end_time: string;
  weeks: number[];
  color?: string;
}

interface UpdateCourseRequest {
  name?: string;
  teacher?: string;
  location?: string;
  weekday?: number;
  start_time?: string;
  end_time?: string;
  weeks?: number[];
  color?: string;
}

interface Schedule {
  courses: Course[];
}

interface PushScheduleRequest {
  courses: CreateCourseRequest[];
  replace: boolean;
}

class ApiService {
  private baseUrl = 'http://localhost:8080';

  setBaseUrl(url: string) {
    this.baseUrl = url.replace(/\/$/, ''); // Remove trailing slash
  }

  private async request<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
    const url = `${this.baseUrl}/api/v1${endpoint}`;
    
    const defaultHeaders = {
      'Content-Type': 'application/json',
    };

    const config: RequestInit = {
      ...options,
      headers: {
        ...defaultHeaders,
        ...options.headers,
      },
    };

    try {
      const response = await fetch(url, config);
      
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      const data = await response.json();
      return data;
    } catch (error) {
      console.error('API request failed:', error);
      throw error;
    }
  }

  // 获取课程表
  async getSchedule(): Promise<Schedule> {
    return this.request<Schedule>('/schedule');
  }

  // 创建课程
  async createCourse(course: CreateCourseRequest): Promise<Course> {
    return this.request<Course>('/courses', {
      method: 'POST',
      body: JSON.stringify(course),
    });
  }

  // 更新课程
  async updateCourse(id: string, course: UpdateCourseRequest): Promise<Course> {
    return this.request<Course>(`/courses/${id}`, {
      method: 'PUT',
      body: JSON.stringify(course),
    });
  }

  // 删除课程
  async deleteCourse(id: string): Promise<Course> {
    return this.request<Course>(`/courses/${id}`, {
      method: 'DELETE',
    });
  }

  // 推送课程表（第三方接口）
  async pushSchedule(request: PushScheduleRequest): Promise<Schedule> {
    return this.request<Schedule>('/schedule/push', {
      method: 'POST',
      body: JSON.stringify(request),
    });
  }
}

export const apiService = new ApiService();
export type { Course, CreateCourseRequest, UpdateCourseRequest, Schedule, PushScheduleRequest };
