use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use log::{error, info};

mod database;
mod db_storage;
mod handlers;
mod models;
mod schema;
// mod storage; // 旧的内存存储，已被数据库存储替代

use handlers::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    // 初始化日志系统
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("🚀 课程表后端服务启动中...");

    // 初始化数据库
    info!("📊 初始化数据库连接...");
    let mut connection = database::establish_connection();
    database::run_migrations(&mut connection);
    info!("✅ 数据库初始化完成");

    let server = HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new().wrap(cors).wrap(Logger::default()).service(
            web::scope("/api/v1")
                .service(get_schedule)
                .service(create_course)
                .service(update_course)
                .service(delete_course)
                .service(push_schedule),
        )
    })
    .bind("127.0.0.1:8080");

    match server {
        Ok(server) => {
            info!("🚀 服务器启动完成，等待请求...");

            server.run().await
        }
        Err(e) => {
            error!("❌ 服务器绑定失败: {}", e);
            error!("💡 请检查端口 8080 是否被占用");
            Err(e)
        }
    }
}
