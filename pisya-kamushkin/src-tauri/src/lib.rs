// Импорт типов, необходимых для migrations
use tauri_plugin_sql::{Migration, MigrationKind};

//Аннотация необходимая Tauri для мобильных платформ
// На Windows она не мешает
#[cfg_attr(mobile, tauri::mobile_entry_point)]

//Главная функция для запуска приложения
pub fn run(){
    // Создание списка миграций
    let migrations = vec![
        // описание первой миграции
        Migration {
            version: 1,

            description: "create_message_table",

            //Берём SQL запрос из нашего файла
            sql:include_str!("../migrations/0001_initial.sql"),

            // up обозначает что база сдвинется вперёд
            kind: MigrationKind::Up,
        },
    ];

    //Сборщик приложения Tauri
    tauri::Builder::default()
    //Подключаем sql плагин
        .plugin(
            //сборщик плагинов
            tauri_plugin_sql::Builder::default()
            //Связываем migrations с базой sql
                .add_migrations("sqlite:messenger.db", migrations)
            // Собираем плагины
                .build()

        )
    // Создаём plugin opener
        .plugin(tauri_plugin_opener::init())
    //Запускаем приложение
        .run(tauri::generate_context!())
         //Если запуск завершился ошибкой, то сообщаем об этом
    .expect("ошибка при сборке приложения");
}