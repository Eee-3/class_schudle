use core::panic;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // 如果是 SQLite 文件，确保目录存在
    if database_url.starts_with("sqlite:") {
        let file_path = database_url.strip_prefix("sqlite:").unwrap();
        println!("尝试连接数据库文件: {}", file_path);

        if let Some(parent) = std::path::Path::new(file_path).parent() {
            std::fs::create_dir_all(parent).expect("Failed to create database directory");
        }

        // 检查文件是否存在
        if !std::path::Path::new(file_path).exists() {
            println!("数据库文件不存在，将创建: {}", file_path);
        } else {
            println!("数据库文件已存在: {}", file_path);
        }
        SqliteConnection::establish(&file_path)
            .unwrap_or_else(|e| panic!("Error connecting to {}: {}", file_path, e))
    } else {
        panic!("Only sqlite is supported");
    }
}

pub fn run_migrations(connection: &mut SqliteConnection) {
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
}
