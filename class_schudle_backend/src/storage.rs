use crate::models::Course;
use log::{debug, info};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub type Storage = Arc<Mutex<HashMap<Uuid, Course>>>;

pub fn create_storage() -> Storage {
    info!("💾 初始化内存存储");
    Arc::new(Mutex::new(HashMap::new()))
}

pub fn get_all_courses(storage: &Storage) -> Vec<Course> {
    let courses = storage.lock().unwrap();
    let course_list: Vec<Course> = courses.values().cloned().collect();
    debug!("📖 获取所有课程: {} 门", course_list.len());
    course_list
}

pub fn get_course_by_id(storage: &Storage, id: &Uuid) -> Option<Course> {
    let courses = storage.lock().unwrap();
    courses.get(id).cloned()
}

pub fn insert_course(storage: &Storage, course: Course) -> Course {
    let mut courses = storage.lock().unwrap();
    courses.insert(course.id, course.clone());
    debug!("💾 课程已存储: {} (ID: {})", course.name, course.id);
    course
}

pub fn update_course(storage: &Storage, id: &Uuid, updated_course: Course) -> Option<Course> {
    let mut courses = storage.lock().unwrap();
    if courses.contains_key(id) {
        courses.insert(*id, updated_course.clone());
        Some(updated_course)
    } else {
        None
    }
}

pub fn delete_course(storage: &Storage, id: &Uuid) -> Option<Course> {
    let mut courses = storage.lock().unwrap();
    courses.remove(id)
}

pub fn clear_all_courses(storage: &Storage) {
    let mut courses = storage.lock().unwrap();
    let count = courses.len();
    courses.clear();
    info!("🗑️ 清空所有课程: {} 门课程已删除", count);
}
