use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::courses;

// 数据库模型 - 用于从数据库查询
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = courses)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Course {
    pub id: String,
    pub name: String,
    pub teacher: Option<String>,
    pub location: Option<String>,
    pub weekday: i32,       // 1=Monday, 2=Tuesday, ..., 7=Sunday
    pub start_time: String, // 存储为 "HH:MM:SS" 格式
    pub end_time: String,   // 存储为 "HH:MM:SS" 格式
    pub weeks: String,      // JSON 字符串存储周次信息
    pub color: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// 插入模型 - 用于插入数据库
#[derive(Debug, Insertable)]
#[diesel(table_name = courses)]
pub struct NewCourse {
    pub id: String,
    pub name: String,
    pub teacher: Option<String>,
    pub location: Option<String>,
    pub weekday: i32,
    pub start_time: String,
    pub end_time: String,
    pub weeks: String,
    pub color: Option<String>,
}

// 更新模型 - 用于更新数据库
#[derive(Debug, AsChangeset)]
#[diesel(table_name = courses)]
pub struct UpdateCourse {
    pub name: Option<String>,
    pub teacher: Option<String>,
    pub location: Option<String>,
    pub weekday: Option<i32>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub weeks: Option<String>,
    pub color: Option<String>,
    pub updated_at: NaiveDateTime,
}

// API 响应模型 - 用于前端交互
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseResponse {
    pub id: String,
    pub name: String,
    pub teacher: Option<String>,
    pub location: Option<String>,
    pub weekday: i32,
    pub start_time: String, // "HH:MM:SS" 格式
    pub end_time: String,   // "HH:MM:SS" 格式
    pub weeks: Vec<i32>,    // 解析后的周次数组
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCourseRequest {
    pub name: String,
    pub teacher: Option<String>,
    pub location: Option<String>,
    pub weekday: i32,       // 1=Monday, 2=Tuesday, ..., 7=Sunday
    pub start_time: String, // "HH:MM:SS" 格式
    pub end_time: String,   // "HH:MM:SS" 格式
    pub weeks: Vec<i32>,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCourseRequest {
    pub name: Option<String>,
    pub teacher: Option<String>,
    pub location: Option<String>,
    pub weekday: Option<i32>,       // 1=Monday, 2=Tuesday, ..., 7=Sunday
    pub start_time: Option<String>, // "HH:MM:SS" 格式
    pub end_time: Option<String>,   // "HH:MM:SS" 格式
    pub weeks: Option<Vec<i32>>,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Schedule {
    pub courses: Vec<CourseResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PushScheduleRequest {
    pub courses: Vec<CreateCourseRequest>,
    pub replace: bool, // 是否替换现有课表
}

// 工具函数：将数据库模型转换为 API 响应模型
impl From<Course> for CourseResponse {
    fn from(course: Course) -> Self {
        let weeks: Vec<i32> = serde_json::from_str(&course.weeks).unwrap_or_default();
        CourseResponse {
            id: course.id,
            name: course.name,
            teacher: course.teacher,
            location: course.location,
            weekday: course.weekday,
            start_time: course.start_time,
            end_time: course.end_time,
            weeks,
            color: course.color,
        }
    }
}
