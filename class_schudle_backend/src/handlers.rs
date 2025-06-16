use crate::models::{CreateCourseRequest, PushScheduleRequest, Schedule, UpdateCourseRequest};
use actix_web::{delete, get, post, put, web, HttpResponse, Result};
use log::{debug, error, info, warn};

#[get("/schedule")]
pub async fn get_schedule() -> Result<HttpResponse> {
    info!("📋 获取课程表请求");

    match crate::db_storage::get_all_courses() {
        Ok(courses) => {
            let schedule = Schedule { courses };
            info!("✅ 返回 {} 门课程", schedule.courses.len());
            Ok(HttpResponse::Ok().json(schedule))
        }
        Err(e) => {
            error!("❌ 获取课程表失败: {}", e);
            Ok(HttpResponse::InternalServerError().json("Failed to get schedule"))
        }
    }
}

#[post("/courses")]
pub async fn create_course(course_req: web::Json<CreateCourseRequest>) -> Result<HttpResponse> {
    info!("➕ 创建课程请求: {}", course_req.name);
    debug!(
        "课程详情: 教师={:?}, 地点={:?}, 星期={}, 时间={}~{}",
        course_req.teacher,
        course_req.location,
        course_req.weekday,
        course_req.start_time,
        course_req.end_time
    );

    match crate::db_storage::insert_course(&course_req) {
        Ok(created_course) => {
            info!(
                "✅ 课程创建成功: {} (ID: {})",
                created_course.name, created_course.id
            );
            Ok(HttpResponse::Created().json(created_course))
        }
        Err(e) => {
            error!("❌ 课程创建失败: {}", e);
            Ok(HttpResponse::InternalServerError().json("Failed to create course"))
        }
    }
}

#[put("/courses/{id}")]
pub async fn update_course(
    path: web::Path<String>,
    update_req: web::Json<UpdateCourseRequest>,
) -> Result<HttpResponse> {
    let course_id = path.into_inner();
    info!("📝 更新课程请求: ID={}", course_id);

    match crate::db_storage::update_course(&course_id, &update_req) {
        Ok(Some(updated_course)) => {
            info!("✅ 课程更新成功: {}", updated_course.name);
            Ok(HttpResponse::Ok().json(updated_course))
        }
        Ok(None) => {
            warn!("⚠️ 课程未找到: ID={}", course_id);
            Ok(HttpResponse::NotFound().json("Course not found"))
        }
        Err(e) => {
            error!("❌ 课程更新失败: {}", e);
            Ok(HttpResponse::InternalServerError().json("Failed to update course"))
        }
    }
}

#[delete("/courses/{id}")]
pub async fn delete_course(path: web::Path<String>) -> Result<HttpResponse> {
    let course_id = path.into_inner();
    info!("🗑️ 删除课程请求: ID={}", course_id);

    match crate::db_storage::delete_course(&course_id) {
        Ok(true) => {
            info!("✅ 课程删除成功: ID={}", course_id);
            Ok(HttpResponse::Ok().json("Course deleted successfully"))
        }
        Ok(false) => {
            warn!("⚠️ 要删除的课程未找到: ID={}", course_id);
            Ok(HttpResponse::NotFound().json("Course not found"))
        }
        Err(e) => {
            error!("❌ 课程删除失败: {}", e);
            Ok(HttpResponse::InternalServerError().json("Failed to delete course"))
        }
    }
}

#[post("/schedule/push")]
pub async fn push_schedule(push_req: web::Json<PushScheduleRequest>) -> Result<HttpResponse> {
    info!(
        "📤 推送课程表请求: {} 门课程, 替换模式={}",
        push_req.courses.len(),
        push_req.replace
    );

    if push_req.replace {
        info!("🔄 清空现有课程表");
        match crate::db_storage::delete_all_courses() {
            Ok(deleted_count) => {
                info!("✅ 已清空 {} 门课程", deleted_count);
            }
            Err(e) => {
                error!("❌ 清空课程表失败: {}", e);
                return Ok(HttpResponse::InternalServerError().json("Failed to clear courses"));
            }
        }
    }

    match crate::db_storage::insert_multiple_courses(&push_req.courses) {
        Ok(created_courses) => {
            info!(
                "✅ 课程表推送完成: 成功创建 {} 门课程",
                created_courses.len()
            );
            Ok(HttpResponse::Ok().json(Schedule {
                courses: created_courses,
            }))
        }
        Err(e) => {
            error!("❌ 批量创建课程失败: {}", e);
            Ok(HttpResponse::InternalServerError().json("Failed to create courses"))
        }
    }
}
