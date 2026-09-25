CREATE TABLE IF NOT EXISTS users (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  username TEXT NOT NULL UNIQUE,
  display_name TEXT NOT NULL,
  avatar_path TEXT,
  status TEXT NOT NULL DEFAULT '',
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT OR IGNORE INTO users(
       id, username, display_name, status
)
       VALUES (
       1,
       'oleg227',
       'Олег',
       'В сети'
       );
INSERT OR IGNORE INTO users(
       id, username, display_name, status
)
       VALUES (
       1,
       'kirill2010',
       'Кирилл',
       'В сети'
       );
INSERT OR IGNORE INTO users(
       id, username, display_name, status
)
       VALUES (
       1,
       'mishaLox',
       'Миша',
       'В сети'
       );

INSERT OR IGNORE INTO users(
       username,
       display_name
)
SELECT
    -- технический username legacy_1 legacy_2 ....
    -- CAST превращает число в текст
    'legacy_' || CAST(old_author.first_message_id AS TEXT),
    old_authors.author
FROM (
     SELECT
         --  MIN(id) берём самый малый id сообщения для этого пользователя
         MIN(id) AS first_message_id,
            author
        FROM messages
        -- создаём одну группу для каждого имени автора
        GROUP BY author
     )
WHERE NOT EXISTS(
    SELECT  1
    FROM users
    WHERE users.display_name = old_authors.author
)


CREATE TABLE message_new(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    chat_id INTEGER NOT NULL
    REFERENCES chats(id)
    ON DELETE CASCADE, --связаные с ним сообщения также будут удалены

    author_id INTEGER NOT NULL
            REFERENCES user(id) -- ссылка на users.id
            ON DELETE  RESTRICT -- нельзя удалить пользователя, если на него ссылаются сообщения

    type TEXT NOT NULL DEFAULT 'text',

    body TEXT,

    attachment TEXT,

    created_at TEXT NOT NULL DEFAULT  CURRENT_TIMESTAMP,

    -- Проверяет и разрешает только типы из данного списка
    CHECK (
        type IN(
            'text',
            'image'
            )
        )
);
INSERT INTO message_new(
   id, chat_id, author_id, body,
  attachment, created_at)
SELECT
    -- Оставляем старый id тем же
    messages.id,
    -- Оставляем старый chat_id
    messages.chat_id,
    -- Вместо старого текстового author ищем настоящий users.id
(
SELECT users.id
FROM users

WHERE users.display_name = messages.author
    -- Если есть одинаковый display_name берём пользователя с меньшим id
ORDER BY users.id ASC
LIMIT 1
),
messages.type,
    messages.body,
    messages.attachment,
    messages.created_at,

FROM messages;

DROP TABLE messages;

ALTER TABLE messages_new
    RENAME TO messages;

-- Индексы нужны для быстрого поиска по сообщению или автору
CREATE INDEX if NOT EXISTS
idx_messages_author_id
ON messages(chat_id);

CREATE INDEX if NOT EXISTS
    idx_messages_author_id
    ON messages(author_id);