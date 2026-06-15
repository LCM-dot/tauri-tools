// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::PathBuf;
use tauri::Manager;

/// 获取安全的文件路径（防路径穿越攻击）
/// 确保用户传入的 filename 不会跳出 AppData 目录
fn get_safe_file_path(app: &tauri::AppHandle, filename: &str) -> Result<PathBuf, String> {
    // 获取应用本地数据目录 (Windows 下通常是 AppData\Local\com.xxx.app)
    let app_dir = app
        .path()
        .app_local_data_dir()
        .map_err(|e| format!("无法获取应用数据目录: {}", e))?;

    // 确保目录存在，如果不存在则创建
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir)
            .map_err(|e| format!("无法创建应用数据目录: {}", e))?;
    }

    let file_path = app_dir.join(filename);

    // 安全检查：确保最终路径依然在 app_dir 内部，防止 "../../../etc/passwd" 这种攻击
    if !file_path.starts_with(&app_dir) {
        return Err("非法操作：检测到路径穿越尝试！".to_string());
    }

    Ok(file_path)
}

/// 写入文本文件到应用本地数据目录
#[tauri::command]
fn write_text_file(app: tauri::AppHandle, filename: String, content: String) -> Result<String, String> {
    let file_path = get_safe_file_path(&app, &filename)?;

    // 执行写入操作
    fs::write(&file_path, content)
        .map_err(|e| format!("写入文件失败: {}", e))?;

    // 返回物理路径的字符串形式给前端
    Ok(file_path.to_string_lossy().into_owned())
}

/// 从应用本地数据目录读取文本文件
#[tauri::command]
fn read_text_file(app: tauri::AppHandle, filename: String) -> Result<String, String> {
    let file_path = get_safe_file_path(&app, &filename)?;

    // 检查文件是否存在
    if !file_path.exists() {
        return Err(format!("情报不存在: 找不到文件 {}", filename));
    }

    // 执行读取操作
    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("读取文件失败: {}", e))?;

    Ok(content)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // 注册前后端通信网关
        .invoke_handler(tauri::generate_handler![
            write_text_file,
            read_text_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}