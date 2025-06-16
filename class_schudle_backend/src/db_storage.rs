use chrono::Utc;
use diesel::prelude::*;
use log::{debug, info};
use uuid::Uuid;

use crate::database::establish_connection;
use crate::models::{
    Course, CourseResponse, CreateCourseRequest, NewCourse, UpdateCourse, UpdateCourseRequest,
};
use crate::schema::courses;

pub fn get_all_courses() -> Result<Vec<CourseResponse>, diesel::result::Error> {
    let mut connection = establish_connection();

    let results = courses::table
        .select(Course::as_select())
        .load(&mut connection)?;

    let course_responses: Vec<CourseResponse> =
        results.into_iter().map(|course| course.into()).collect();

    info!("📋 从数据库获取到 {} 门课程", course_responses.len());
    Ok(course_responses)
}

pub fn get_course_by_id(course_id: &str) -> Result<Option<CourseResponse>, diesel::result::Error> {
    let mut connection = establish_connection();

    let result = courses::table
        .filter(courses::id.eq(course_id))
        .select(Course::as_select())
        .first(&mut connection)
        .optional()?;

    match result {
        Some(course) => {
            debug!("🔍 找到课程: {} (ID: {})", course.name, course.id);
            Ok(Some(course.into()))
        }
        None => {
            debug!("❌ 未找到课程 ID: {}", course_id);
            Ok(None)
        }
    }
}

pub fn insert_course(
    course_req: &CreateCourseRequest,
) -> Result<CourseResponse, diesel::result::Error> {
    let mut connection = establish_connection();

    let course_id = Uuid::new_v4().to_string();
    let weeks_json = serde_json::to_string(&course_req.weeks).unwrap_or_default();

    let new_course = NewCourse {
        id: course_id.clone(),
        name: course_req.name.clone(),
        teacher: course_req.teacher.clone(),
        location: course_req.location.clone(),
        weekday: course_req.weekday,
        start_time: course_req.start_time.clone(),
        end_time: course_req.end_time.clone(),
        weeks: weeks_json,
        color: course_req.color.clone(),
    };

    diesel::insert_into(courses::table)
        .values(&new_course)
        .execute(&mut connection)?;

    // 获取插入的课程
    let inserted_course = courses::table
        .filter(courses::id.eq(&course_id))
        .select(Course::as_select())
        .first(&mut connection)?;

    info!(
        "💾 课程已存储: {} (ID: {})",
        inserted_course.name, inserted_course.id
    );
    Ok(inserted_course.into())
}

pub fn update_course(
    course_id: &str,
    update_req: &UpdateCourseRequest,
) -> Result<Option<CourseResponse>, diesel::result::Error> {
    let mut connection = establish_connection();

    // 首先检查课程是否存在
    let existing_course = courses::table
        .filter(courses::id.eq(course_id))
        .select(Course::as_select())
        .first(&mut connection)
        .optional()?;

    if existing_course.is_none() {
        return Ok(None);
    }

    let weeks_json = update_req
        .weeks
        .as_ref()
        .map(|weeks| serde_json::to_string(weeks).unwrap_or_default());

    let update_course = UpdateCourse {
        name: update_req.name.clone(),
        teacher: update_req.teacher.clone(),
        location: update_req.location.clone(),
        weekday: update_req.weekday,
        start_time: update_req.start_time.clone(),
        end_time: update_req.end_time.clone(),
        weeks: weeks_json,
        color: update_req.color.clone(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::update(courses::table.filter(courses::id.eq(course_id)))
        .set(&update_course)
        .execute(&mut connection)?;

    // 获取更新后的课程
    let updated_course = courses::table
        .filter(courses::id.eq(course_id))
        .select(Course::as_select())
        .first(&mut connection)?;

    info!(
        "🔄 课程已更新: {} (ID: {})",
        updated_course.name, updated_course.id
    );
    Ok(Some(updated_course.into()))
}

pub fn delete_course(course_id: &str) -> Result<bool, diesel::result::Error> {
    let mut connection = establish_connection();

    let deleted_rows = diesel::delete(courses::table.filter(courses::id.eq(course_id)))
        .execute(&mut connection)?;

    if deleted_rows > 0 {
        info!("🗑️ 课程已删除 (ID: {})", course_id);
        Ok(true)
    } else {
        debug!("❌ 未找到要删除的课程 ID: {}", course_id);
        Ok(false)
    }
}

pub fn delete_all_courses() -> Result<usize, diesel::result::Error> {
    let mut connection = establish_connection();

    let deleted_count = diesel::delete(courses::table).execute(&mut connection)?;

    info!("🗑️ 已删除所有课程，共 {} 门", deleted_count);
    Ok(deleted_count)
}

pub fn insert_multiple_courses(
    course_requests: &[CreateCourseRequest],
) -> Result<Vec<CourseResponse>, diesel::result::Error> {
    let mut connection = establish_connection();
    let mut created_courses = Vec::new();

    // 使用事务确保数据一致性
    connection.transaction::<_, diesel::result::Error, _>(|conn| {
        for course_req in course_requests {
            let course_id = Uuid::new_v4().to_string();
            let weeks_json = serde_json::to_string(&course_req.weeks).unwrap_or_default();

            let new_course = NewCourse {
                id: course_id.clone(),
                name: course_req.name.clone(),
                teacher: course_req.teacher.clone(),
                location: course_req.location.clone(),
                weekday: course_req.weekday,
                start_time: course_req.start_time.clone(),
                end_time: course_req.end_time.clone(),
                weeks: weeks_json,
                color: course_req.color.clone(),
            };

            diesel::insert_into(courses::table)
                .values(&new_course)
                .execute(conn)?;

            // 获取插入的课程
            let inserted_course = courses::table
                .filter(courses::id.eq(&course_id))
                .select(Course::as_select())
                .first(conn)?;

            created_courses.push(inserted_course.into());
        }

        Ok(())
    })?;

    info!("💾 批量创建了 {} 门课程", created_courses.len());
    Ok(created_courses)
}
