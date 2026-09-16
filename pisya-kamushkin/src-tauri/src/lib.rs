// Импорт типов, необходимых для migrations
use tauri_plugin_sql::{Migration, MigrationKind};

// Аннотация небходимая Tauri для мобильных платформ
// На Win она не мешает
#[cfg_attr(mobile, tauri::mobile_entry_point)]

#[tauri::command]
fn save_attachment(source: String) -> Result<String, String>{
    let app_dir = std::env::current_dir()
        .map_err(|e| e.to_string())?;
    let attachment_dir = app_dir.join("attachment");
    std::fs::create_dir_all(&attachment_dir)
    .map_err(|e| e.to_string())?;

    let source_path = std::path::Path::new(&source);
    let extension = source_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("png");

    let file_name = format!("image_{}.{}", chrono::Utc::now().timestamp(), extension);

    let destination = attachment_dir.join(&file_name);

    std::fs::copy(source, &destination).map_err(|e| e.to_string())?;

    Ok(
        format!(
            "attachment/{}",
            file_name.to_string()
        )
    )

}

// Главная функция для запуска приложения
pub fn run() {
    // Создание списка миграций
    let migrations = vec![
        // Описание первой миграции
        Migration {
            version: 1,

            description: "create_message_table",

            // Берем SQL запрос из нашего файла
            sql: include_str!("../migrations/0001_initial.sql"),

            // up означает, что база сдвинется вперед
            kind: MigrationKind::Up,
        },
        Migration {
            version: 2,
            description: "create_chats",
            sql: include_str!("../migrations/0002_chats.sql"),
            kind: MigrationKind::Up,
        },
        Migration {
            version: 3,
            description: "message_attachment",
            sql: include_str!("../migrations/0003_message_attachments.sql"),
            kind: MigrationKind::Up,
        }
    ];

    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:messenger.db", migrations)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            save_attachment
        ])
        .run(tauri::generate_context!())
        .expect("Ошибка при запуске приложения");
}
