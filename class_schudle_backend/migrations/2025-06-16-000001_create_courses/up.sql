-- 创建课程表
CREATE TABLE courses (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    teacher TEXT,
    location TEXT,
    weekday INTEGER NOT NULL CHECK (weekday >= 1 AND weekday <= 7),
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL,
    weeks TEXT NOT NULL, -- JSON 数组存储周次信息
    color TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 创建索引以提高查询性能
CREATE INDEX idx_courses_weekday ON courses(weekday);
CREATE INDEX idx_courses_start_time ON courses(start_time);
CREATE INDEX idx_courses_created_at ON courses(created_at);

-- 创建触发器自动更新 updated_at 字段
CREATE TRIGGER update_courses_updated_at 
    AFTER UPDATE ON courses
    FOR EACH ROW
    WHEN NEW.updated_at = OLD.updated_at
BEGIN
    UPDATE courses SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;
