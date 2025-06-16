-- 删除触发器
DROP TRIGGER IF EXISTS update_courses_updated_at;

-- 删除索引
DROP INDEX IF EXISTS idx_courses_created_at;
DROP INDEX IF EXISTS idx_courses_start_time;
DROP INDEX IF EXISTS idx_courses_weekday;

-- 删除课程表
DROP TABLE IF EXISTS courses;
