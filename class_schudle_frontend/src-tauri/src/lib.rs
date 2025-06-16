use log::{debug, error, info, warn};
use std::fs;
use std::path::PathBuf;
use tauri::{Manager, menu::{MenuBuilder, MenuItem, PredefinedMenuItem}, tray::{TrayIconBuilder, TrayIconEvent}};

#[cfg(target_os = "windows")]
use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{
        GetWindowLongW, SetWindowLongW, SetWindowPos, ShowWindow, GWL_EXSTYLE, HWND_BOTTOM,
        SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SW_SHOWNOACTIVATE, WS_EX_NOACTIVATE,
        WS_EX_TOOLWINDOW,
    },
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 显示主应用
#[tauri::command]
fn show_main_app(app: tauri::AppHandle) -> Result<(), String> {
    info!("show_main_app 被调用");

    if let Some(main_window) = app.get_webview_window("main") {
        debug!("找到主窗口，准备显示");
        main_window.show().map_err(|e| {
            error!("显示主窗口失败: {}", e);
            e.to_string()
        })?;
        main_window.set_focus().map_err(|e| {
            error!("设置主窗口焦点失败: {}", e);
            e.to_string()
        })?;
        info!("主窗口已显示并获得焦点");
    } else {
        warn!("未找到主窗口");
        return Err("未找到主窗口".to_string());
    }
    Ok(())
}

// 设置窗口为桌面小组件模式（Windows）
#[tauri::command]
#[cfg(target_os = "windows")]
fn set_desktop_widget_mode(app: tauri::AppHandle) -> Result<(), String> {
    info!("set_desktop_widget_mode 被调用");

    if let Some(widget_window) = app.get_webview_window("widget") {
        let hwnd = widget_window.hwnd().map_err(|e| e.to_string())?;
        let hwnd = HWND(hwnd.0);
        debug!("设置桌面小组件模式，窗口句柄: {:?}", hwnd);

        unsafe {
            // 设置窗口扩展样式，使其不会被激活
            let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
            debug!("当前扩展样式: 0x{:x}", ex_style);

            let new_style = ex_style | WS_EX_NOACTIVATE.0 as i32 | WS_EX_TOOLWINDOW.0 as i32;
            debug!("设置新扩展样式: 0x{:x}", new_style);

            SetWindowLongW(hwnd, GWL_EXSTYLE, new_style);

            // 将窗口设置到桌面层级
            let _ = SetWindowPos(
                hwnd,
                HWND_BOTTOM,
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
            );
            debug!("窗口已设置到桌面层级");

            // 显示窗口但不激活
            let _ = ShowWindow(hwnd, SW_SHOWNOACTIVATE);
            debug!("窗口已显示但未激活");
        }
    } else {
        warn!("未找到 widget 窗口");
    }
    Ok(())
}

// 非 Windows 平台的空实现
#[tauri::command]
#[cfg(not(target_os = "windows"))]
fn set_desktop_widget_mode(_app: tauri::AppHandle) -> Result<(), String> {
    Ok(())
}

// 获取配置文件路径
fn get_config_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&app_data_dir).map_err(|e| e.to_string())?;
    Ok(app_data_dir.join("widget_position.json"))
}

// 保存小组件位置
#[tauri::command]
fn save_widget_position(app: tauri::AppHandle, x: i32, y: i32) -> Result<(), String> {
    info!("保存小组件位置: x={}, y={}", x, y);

    let config_path = get_config_path(&app)?;
    let position_data = serde_json::json!({
        "x": x,
        "y": y
    });

    fs::write(&config_path, position_data.to_string()).map_err(|e| {
        error!("保存位置失败: {}", e);
        e.to_string()
    })?;

    debug!("位置已保存到: {:?}", config_path);
    Ok(())
}

// 恢复小组件位置
#[tauri::command]
fn restore_widget_position(app: tauri::AppHandle) -> Result<(i32, i32), String> {
    let config_path = get_config_path(&app)?;

    if !config_path.exists() {
        info!("位置配置文件不存在，使用默认位置");
        return Ok((100, 100)); // 默认位置
    }

    let content = fs::read_to_string(&config_path).map_err(|e| {
        warn!("读取位置配置失败: {}", e);
        e.to_string()
    })?;

    let position: serde_json::Value = serde_json::from_str(&content).map_err(|e| {
        warn!("解析位置配置失败: {}", e);
        e.to_string()
    })?;

    let x = position["x"].as_i64().unwrap_or(100) as i32;
    let y = position["y"].as_i64().unwrap_or(100) as i32;

    info!("恢复小组件位置: x={}, y={}", x, y);
    Ok((x, y))
}

// 获取当前窗口位置
#[tauri::command]
fn get_window_position(app: tauri::AppHandle) -> Result<(i32, i32), String> {
    if let Some(widget_window) = app.get_webview_window("widget") {
        let position = widget_window.outer_position().map_err(|e| e.to_string())?;
        let x = position.x;
        let y = position.y;
        debug!("获取当前窗口位置: x={}, y={}", x, y);
        Ok((x, y))
    } else {
        Err("未找到 widget 窗口".to_string())
    }
}

// 显示桌面小组件
#[tauri::command]
fn show_widget(app: tauri::AppHandle) -> Result<(), String> {
    info!("show_widget 被调用");

    if let Some(widget_window) = app.get_webview_window("widget") {
        debug!("找到小组件窗口，准备显示");
        widget_window.show().map_err(|e| {
            error!("显示小组件窗口失败: {}", e);
            e.to_string()
        })?;
        info!("小组件窗口已显示");

        // 重新设置桌面模式
        let _ = set_desktop_widget_mode(app);
    } else {
        warn!("未找到小组件窗口");
        return Err("未找到小组件窗口".to_string());
    }
    Ok(())
}

// 隐藏桌面小组件
#[tauri::command]
fn hide_widget(app: tauri::AppHandle) -> Result<(), String> {
    info!("hide_widget 被调用");

    if let Some(widget_window) = app.get_webview_window("widget") {
        debug!("找到小组件窗口，准备隐藏");
        widget_window.hide().map_err(|e| {
            error!("隐藏小组件窗口失败: {}", e);
            e.to_string()
        })?;
        info!("小组件窗口已隐藏");
    } else {
        warn!("未找到小组件窗口");
        return Err("未找到小组件窗口".to_string());
    }
    Ok(())
}

// 切换桌面小组件显示状态
#[tauri::command]
fn toggle_widget(app: tauri::AppHandle) -> Result<bool, String> {
    info!("toggle_widget 被调用");

    if let Some(widget_window) = app.get_webview_window("widget") {
        let is_visible = widget_window.is_visible().map_err(|e| {
            error!("检查小组件窗口可见性失败: {}", e);
            e.to_string()
        })?;

        debug!("当前小组件窗口可见性: {}", is_visible);

        if is_visible {
            hide_widget(app)?;
            info!("小组件已切换为隐藏状态");
            Ok(false)
        } else {
            show_widget(app)?;
            info!("小组件已切换为显示状态");
            Ok(true)
        }
    } else {
        warn!("未找到小组件窗口");
        Err("未找到小组件窗口".to_string())
    }
}

// 退出应用
#[tauri::command]
fn quit_app(_app: tauri::AppHandle) -> Result<(), String> {
    info!("quit_app 被调用，准备退出应用");
    std::process::exit(0);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            show_main_app,
            set_desktop_widget_mode,
            save_widget_position,
            restore_widget_position,
            get_window_position,
            show_widget,
            hide_widget,
            toggle_widget,
            quit_app
        ])
        .setup(|app| {
            // 初始化日志，设置默认级别为 info
            env_logger::Builder::from_default_env()
                .filter_level(log::LevelFilter::Info)
                .init();
            info!("应用启动，初始化日志系统");

            // 创建托盘菜单
            let show_main = MenuItem::with_id(app, "show_main", "显示主窗口", true, None::<&str>)?;
            let toggle_widget_menu = MenuItem::with_id(app, "toggle_widget", "显示/隐藏小组件", true, None::<&str>)?;
            let separator = PredefinedMenuItem::separator(app)?;
            let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

            let menu = MenuBuilder::new(app)
                .items(&[&show_main, &toggle_widget_menu, &separator, &quit])
                .build()?;

            // 创建托盘图标
            let _tray = TrayIconBuilder::with_id("main")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .tooltip("课程表管理系统")
                .on_menu_event(move |app, event| {
                    match event.id.as_ref() {
                        "show_main" => {
                            info!("托盘菜单：显示主窗口");
                            let _ = show_main_app(app.clone());
                        }
                        "toggle_widget" => {
                            info!("托盘菜单：切换小组件");
                            let _ = toggle_widget(app.clone());
                        }
                        "quit" => {
                            info!("托盘菜单：退出应用");
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    match event {
                        TrayIconEvent::Click { button: tauri::tray::MouseButton::Left, .. } => {
                            info!("托盘图标左键点击");
                            let app = tray.app_handle();
                            let _ = show_main_app(app.clone());
                        }
                        TrayIconEvent::DoubleClick { button: tauri::tray::MouseButton::Left, .. } => {
                            info!("托盘图标双击");
                            let app = tray.app_handle();
                            let _ = toggle_widget(app.clone());
                        }
                        _ => {}
                    }
                })
                .build(app)?;

            // 启动时恢复小组件位置并设置桌面模式
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

                // 恢复窗口位置
                if let Ok((x, y)) = restore_widget_position(app_handle.clone()) {
                    if let Some(widget_window) = app_handle.get_webview_window("widget") {
                        let _ = widget_window.set_position(tauri::Position::Physical(tauri::PhysicalPosition { x, y }));
                        info!("窗口位置已恢复到: x={}, y={}", x, y);
                    }
                }

                let _ = set_desktop_widget_mode(app_handle);
            });

            // 设置主窗口关闭事件处理
            if let Some(main_window) = app.get_webview_window("main") {
                let main_window_clone = main_window.clone();
                main_window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        info!("主窗口关闭请求，隐藏窗口而不是销毁");
                        // 阻止默认的关闭行为
                        api.prevent_close();
                        // 隐藏窗口
                        let _ = main_window_clone.hide();
                    }
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
