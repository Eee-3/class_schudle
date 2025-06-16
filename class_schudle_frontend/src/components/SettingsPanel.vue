<template>
  <div class="settings-panel">
    <div class="settings-header">
      <h2 class="header-title">
        <el-icon><Setting /></el-icon>
        课程表设置
      </h2>
    </div>

    <div class="settings-content">
      <!-- 后端连接设置 -->
      <el-card class="setting-section" shadow="hover">
        <template #header>
          <div class="section-header">
            <el-icon><Link /></el-icon>
            <span>后端连接</span>
          </div>
        </template>

        <el-form :model="connectionForm" label-width="120px">
          <el-form-item label="服务地址:">
            <el-input
              v-model="backendUrl"
              placeholder="http://localhost:8080"
            >
              <template #prefix>
                <el-icon><Link /></el-icon>
              </template>
            </el-input>
          </el-form-item>

          <el-form-item>
            <el-button
              type="primary"
              @click="testConnection"
              :loading="isConnecting"
            >
              <el-icon><Connection /></el-icon>
              {{ isConnecting ? '连接中...' : '测试连接' }}
            </el-button>
            <el-button
              type="success"
              @click="syncSchedule"
              :disabled="!isConnected"
            >
              <el-icon><Refresh /></el-icon>
              同步课表
            </el-button>
          </el-form-item>

          <el-form-item>
            <el-alert
              :title="connectionStatus"
              :type="connectionAlertType"
              :closable="false"
              show-icon
            />
          </el-form-item>
        </el-form>
      </el-card>

      <!-- 课程管理 -->
      <el-card class="setting-section" shadow="hover">
        <template #header>
          <div class="section-header">
            <el-icon><Document /></el-icon>
            <span>课程管理</span>
            <div class="header-actions">
              <el-button type="primary" @click="showAddCourse = true">
                <el-icon><Plus /></el-icon>
                添加课程
              </el-button>
              <el-button type="success" @click="addTestData">
                <el-icon><DataBoard /></el-icon>
                添加测试数据
              </el-button>
              <el-button type="danger" @click="clearAllCourses">
                <el-icon><Delete /></el-icon>
                清空课表
              </el-button>
            </div>
          </div>
        </template>

        <el-table
          :data="scheduleStore.courses"
          style="width: 100%"
          :empty-text="'暂无课程数据'"
          stripe
        >
          <el-table-column prop="name" label="课程名称" width="120" />
          <el-table-column label="时间" width="180">
            <template #default="scope">
              <el-tag type="info" size="small">
                {{ getWeekdayName(scope.row.weekday) }}
              </el-tag>
              <br>
              <span class="time-text">
                {{ formatTime(scope.row.start_time) }} - {{ formatTime(scope.row.end_time) }}
              </span>
            </template>
          </el-table-column>
          <el-table-column prop="teacher" label="教师" width="100" />
          <el-table-column prop="location" label="地点" width="100" />
          <el-table-column label="周次" width="120">
            <template #default="scope">
              <el-tag size="small" v-if="scope.row.weeks.length <= 3">
                {{ scope.row.weeks.join(',') }}
              </el-tag>
              <el-tag size="small" v-else>
                {{ scope.row.weeks[0] }}-{{ scope.row.weeks[scope.row.weeks.length - 1] }}
              </el-tag>
            </template>
          </el-table-column>
          <el-table-column label="操作" width="150">
            <template #default="scope">
              <el-button
                size="small"
                type="primary"
                @click="editCourse(scope.row)"
              >
                <el-icon><Edit /></el-icon>
                编辑
              </el-button>
              <el-button
                size="small"
                type="danger"
                @click="deleteCourse(scope.row.id)"
              >
                <el-icon><Delete /></el-icon>
                删除
              </el-button>
            </template>
          </el-table-column>
        </el-table>
      </el-card>
    </div>

    <!-- 添加/编辑课程对话框 -->
    <el-dialog
      v-model="showAddCourse"
      :title="editingCourse ? '编辑课程' : '添加课程'"
      width="600px"
      @close="closeModal"
    >
      <el-form
        :model="courseForm"
        :rules="courseRules"
        ref="courseFormRef"
        label-width="100px"
      >
        <el-form-item label="课程名称" prop="name">
          <el-input v-model="courseForm.name" placeholder="请输入课程名称" />
        </el-form-item>

        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="教师" prop="teacher">
              <el-input v-model="courseForm.teacher" placeholder="请输入教师姓名" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="地点" prop="location">
              <el-input v-model="courseForm.location" placeholder="请输入上课地点" />
            </el-form-item>
          </el-col>
        </el-row>

        <el-form-item label="星期" prop="weekday">
          <el-select v-model="courseForm.weekday" placeholder="请选择星期" style="width: 100%">
            <el-option label="星期一" :value="1" />
            <el-option label="星期二" :value="2" />
            <el-option label="星期三" :value="3" />
            <el-option label="星期四" :value="4" />
            <el-option label="星期五" :value="5" />
            <el-option label="星期六" :value="6" />
            <el-option label="星期日" :value="7" />
          </el-select>
        </el-form-item>

        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="开始时间" prop="start_time">
              <el-time-picker
                v-model="courseForm.start_time"
                format="HH:mm"
                value-format="HH:mm"
                placeholder="选择开始时间"
                style="width: 100%"
              />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="结束时间" prop="end_time">
              <el-time-picker
                v-model="courseForm.end_time"
                format="HH:mm"
                value-format="HH:mm"
                placeholder="选择结束时间"
                style="width: 100%"
              />
            </el-form-item>
          </el-col>
        </el-row>

        <el-form-item label="周次" prop="weeks">
          <el-input
            v-model="weeksInput"
            placeholder="请输入周次，用逗号分隔，如: 1,2,3,4,5"
          />
          <div class="form-tip">
            <el-text size="small" type="info">
              示例：1,2,3,4,5 或 1-16 表示第1到16周
            </el-text>
          </div>
        </el-form-item>

        <el-form-item label="颜色" prop="color">
          <el-color-picker v-model="courseForm.color" />
        </el-form-item>
      </el-form>

      <template #footer>
        <el-button @click="closeModal">取消</el-button>
        <el-button type="primary" @click="saveCourse" :loading="scheduleStore.loading">
          {{ editingCourse ? '更新' : '保存' }}
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, reactive } from 'vue';
import { ElMessage, ElMessageBox, type FormInstance, type FormRules } from 'element-plus';
import { scheduleStore } from '../stores/scheduleStore';
import { apiService } from '../services/apiService';
import { getWeekdayName, formatTime } from '../utils/courseUtils';

const backendUrl = ref('http://localhost:8080');
const isConnecting = ref(false);
const isConnected = ref(false);
const connectionStatus = ref('未连接');

const showAddCourse = ref(false);
const editingCourse = ref<any>(null);
const courseFormRef = ref<FormInstance>();

const connectionForm = reactive({
  url: 'http://localhost:8080'
});

const courseForm = reactive({
  name: '',
  teacher: '',
  location: '',
  weekday: 1,
  start_time: '',
  end_time: '',
  color: '#409EFF'
});

const weeksInput = ref('1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16');

const courseRules: FormRules = {
  name: [
    { required: true, message: '请输入课程名称', trigger: 'blur' }
  ],
  weekday: [
    { required: true, message: '请选择星期', trigger: 'change' }
  ],
  start_time: [
    { required: true, message: '请选择开始时间', trigger: 'change' }
  ],
  end_time: [
    { required: true, message: '请选择结束时间', trigger: 'change' }
  ]
};

const connectionAlertType = computed(() => {
  if (isConnected.value) return 'success';
  if (connectionStatus.value.includes('错误')) return 'error';
  return 'warning';
});

async function testConnection() {
  isConnecting.value = true;
  try {
    apiService.setBaseUrl(backendUrl.value);
    await apiService.getSchedule();
    isConnected.value = true;
    connectionStatus.value = '连接成功';
    ElMessage.success('后端连接成功！');
  } catch (error) {
    isConnected.value = false;
    connectionStatus.value = '连接错误: ' + (error as Error).message;
    ElMessage.error('连接失败: ' + (error as Error).message);
  } finally {
    isConnecting.value = false;
  }
}

async function syncSchedule() {
  try {
    await scheduleStore.loadSchedule();
    connectionStatus.value = '课表同步成功';
    ElMessage.success('课表同步成功！');
  } catch (error) {
    connectionStatus.value = '同步失败: ' + (error as Error).message;
    ElMessage.error('同步失败: ' + (error as Error).message);
  }
}

function editCourse(course: any) {
  editingCourse.value = course;
  Object.assign(courseForm, {
    name: course.name,
    teacher: course.teacher || '',
    location: course.location || '',
    weekday: course.weekday,
    start_time: course.start_time.substring(0, 5),
    end_time: course.end_time.substring(0, 5),
    color: course.color || '#409EFF'
  });
  weeksInput.value = course.weeks.join(',');
  showAddCourse.value = true;
}

async function saveCourse() {
  if (!courseFormRef.value) return;

  try {
    await courseFormRef.value.validate();

    const weeks = weeksInput.value.split(',').map(w => parseInt(w.trim())).filter(w => !isNaN(w));

    const courseData = {
      ...courseForm,
      start_time: courseForm.start_time + ':00',
      end_time: courseForm.end_time + ':00',
      weeks
    };

    if (editingCourse.value) {
      await scheduleStore.updateCourse(editingCourse.value.id, courseData);
      ElMessage.success('课程更新成功！');
    } else {
      await scheduleStore.createCourse(courseData);
      ElMessage.success('课程添加成功！');
    }
    closeModal();
  } catch (error) {
    if (error !== false) { // 不是表单验证错误
      ElMessage.error('保存失败: ' + (error as Error).message);
    }
  }
}

function closeModal() {
  showAddCourse.value = false;
  editingCourse.value = null;
  Object.assign(courseForm, {
    name: '',
    teacher: '',
    location: '',
    weekday: 1,
    start_time: '',
    end_time: '',
    color: '#409EFF'
  });
  weeksInput.value = '1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16';
  courseFormRef.value?.resetFields();
}

async function deleteCourse(id: string) {
  try {
    await ElMessageBox.confirm(
      '确定要删除这门课程吗？',
      '删除确认',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning',
      }
    );

    await scheduleStore.deleteCourse(id);
    ElMessage.success('课程删除成功！');
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败: ' + (error as Error).message);
    }
  }
}

async function clearAllCourses() {
  try {
    await ElMessageBox.confirm(
      '确定要清空所有课程吗？此操作不可恢复！',
      '清空确认',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning',
      }
    );

    scheduleStore.clearCourses();
    ElMessage.success('课表已清空！');
  } catch (error) {
    // 用户取消操作
  }
}

async function addTestData() {
  try {
    await ElMessageBox.confirm(
      '确定要添加测试课程数据吗？这将添加一些示例课程。',
      '添加测试数据',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'info',
      }
    );

    const testCourses = [
      {
        name: '高等数学',
        teacher: '张教授',
        location: '教学楼A101',
        weekday: 1, // 周一
        start_time: '08:00:00',
        end_time: '09:40:00',
        weeks: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20],
        color: '#409EFF'
      },
      {
        name: '英语',
        teacher: '李老师',
        location: '教学楼B203',
        weekday: 2, // 周二
        start_time: '10:00:00',
        end_time: '11:40:00',
        weeks: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20],
        color: '#67C23A'
      },
      {
        name: '计算机程序设计',
        teacher: '王老师',
        location: '实验楼C301',
        weekday: 3, // 周三
        start_time: '14:00:00',
        end_time: '15:40:00',
        weeks: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20],
        color: '#E6A23C'
      },
      {
        name: '物理实验',
        teacher: '赵老师',
        location: '物理实验室',
        weekday: 4, // 周四
        start_time: '16:00:00',
        end_time: '17:40:00',
        weeks: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,17, 18, 19, 20],
        color: '#F56C6C'
      },
      {
        name: '体育',
        teacher: '刘教练',
        location: '体育馆',
        weekday: 5, // 周五
        start_time: '09:00:00',
        end_time: '10:40:00',
        weeks: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20],
        color: '#909399'
      }
    ];

    // 批量添加课程
    for (const courseData of testCourses) {
      try {
        await scheduleStore.createCourse(courseData);
      } catch (error) {
        console.error('添加课程失败:', courseData.name, error);
      }
    }

    ElMessage.success('测试数据添加成功！');
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('添加测试数据失败: ' + (error as Error).message);
    }
  }
}

</script>

<style scoped>
.settings-panel {
  padding: 2rem;
  min-height: calc(100vh - 200px);
}

.settings-header {
  margin-bottom: 2rem;
}

.header-title {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  color: #2c3e50;
  font-size: 1.5rem;
  font-weight: 600;
  margin: 0;
  transition: color 0.3s ease;
}

.dark .header-title {
  color: #e5eaf3;
}

.settings-content {
  max-height: calc(100vh - 300px);
  overflow-y: auto;
}

.setting-section {
  margin-bottom: 2rem;
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.5rem;
  font-weight: 600;
  color: #2c3e50;
  transition: color 0.3s ease;
}

.dark .section-header {
  color: #e5eaf3;
}

.header-actions {
  display: flex;
  gap: 0.5rem;
}

.time-text {
  font-size: 0.9rem;
  color: #606266;
  margin-top: 0.25rem;
}

.form-tip {
  margin-top: 0.5rem;
}

:deep(.el-form-item__label) {
  font-weight: 500;
  color: #2c3e50;
}

:deep(.el-table .el-table__cell) {
  padding: 12px 0;
}

:deep(.el-table__empty-text) {
  color: #909399;
}

:deep(.el-dialog__header) {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  margin: 0;
  padding: 20px;
}

:deep(.el-dialog__title) {
  color: white;
  font-weight: 600;
}

:deep(.el-dialog__headerbtn .el-dialog__close) {
  color: white;
}

:deep(.el-dialog__body) {
  padding: 30px 20px;
}

:deep(.el-alert) {
  border-radius: 6px;
}

:deep(.el-card__header) {
  background: rgba(64, 158, 255, 0.1);
  border-bottom: 1px solid #e4e7ed;
}
</style>
