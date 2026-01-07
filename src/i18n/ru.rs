//! Русский - Russian translations

use super::Translations;
use std::collections::HashMap;

pub fn translations() -> Translations {
    let mut data = HashMap::new();

    // Common
    data.insert("app.name", "Лазарус");
    data.insert("app.tagline", "Офлайн Управление Личными Знаниями");
    data.insert("nav.notes", "Заметки");
    data.insert("nav.search", "Поиск");
    data.insert("nav.wiki", "Вики");

    // Home
    data.insert("home.notes", "Заметки");
    data.insert("home.streak", "Дней Подряд");
    data.insert("home.day", "дн.");
    data.insert("home.quick_start", "Быстрый Старт");
    data.insert("home.new_note", "Новая Заметка");
    data.insert("home.note_list", "Список Заметок");
    data.insert("home.split_view", "Разделённый Вид");
    data.insert("home.srs_review", "Повторение SRS");
    data.insert("home.search", "Поиск");
    data.insert("home.zim_manage", "Управление ZIM");
    data.insert("home.security", "Безопасность");
    data.insert("home.shortcuts", "Горячие Клавиши");
    data.insert("home.shortcut.new_note", "Новая заметка");
    data.insert("home.shortcut.search", "Поиск");
    data.insert("home.shortcut.help", "Помощь");

    // Editor
    data.insert("editor.title_placeholder", "Введите заголовок");
    data.insert("editor.content_placeholder", "Начните писать здесь...");
    data.insert("editor.tags", "Теги");
    data.insert("editor.tags_placeholder", "тег1, тег2, ...");
    data.insert("editor.edit_note", "Редактировать Заметку");
    data.insert("editor.encrypt", "Включить шифрование");
    data.insert("editor.focus", "Фокус");
    data.insert("editor.fullscreen", "Полный Экран");
    data.insert("editor.save", "Сохранить");
    data.insert("editor.saved", "Сохранено");
    data.insert("editor.auto_saved", "Авто сохранено");
    data.insert("editor.changed", "Изменено...");
    data.insert("editor.words", "слов");
    data.insert("editor.save_complete", "💾 Сохранено!");
    data.insert("editor.encrypt_on", "🔒 Шифрование включено");
    data.insert("editor.encrypt_off", "🔓 Шифрование выключено");
    data.insert(
        "editor.pin_required",
        "Сначала установите PIN (Меню Безопасность)",
    );
    data.insert("editor.pin_locked", "Сначала разблокируйте PIN");

    // Notes list
    data.insert("notes.title", "Заметки");
    data.insert("notes.export", "Экспорт");
    data.insert("notes.import", "Импорт");
    data.insert("notes.no_notes", "Пока нет заметок");
    data.insert("notes.create_first", "Создайте первую заметку!");
    data.insert("notes.no_title", "Без Названия");
    data.insert("notes.edit", "Изменить");
    data.insert("notes.delete", "Удалить");
    data.insert("notes.delete_confirm", "Вы уверены что хотите удалить?");
    data.insert("notes.encrypted", "Зашифровано");
    data.insert("notes.created", "Создано");
    data.insert("notes.updated", "Обновлено");
    data.insert("notes.find_duplicates", "Найти Дубликаты");
    data.insert("notes.select_all", "Выбрать Все");
    data.insert("notes.selected", "выбрано");
    data.insert("notes.create_package", "Создать Пакет");
    data.insert("notes.duplicates_title", "Дубликаты Заметок");
    data.insert("notes.no_duplicates", "Дубликаты не найдены");
    data.insert("notes.export_package", "Экспорт Пакета");
    data.insert("notes.import_package", "Импорт Пакета");
    data.insert("notes.package_title", "Название Пакета");
    data.insert("notes.package_title_placeholder", "напр., Физика 101");
    data.insert("notes.package_author", "Автор");
    data.insert("notes.package_author_placeholder", "Ваше имя");
    data.insert("notes.package_description", "Описание");
    data.insert(
        "notes.package_description_placeholder",
        "Описание пакета (необязательно)",
    );
    data.insert("notes.drop_file", "Перетащите файл .laz сюда");
    data.insert("notes.or", "или");
    data.insert("notes.select_file", "Выбрать Файл");

    // Search
    data.insert("search.title", "Поиск");
    data.insert("search.placeholder", "Введите поисковый запрос...");
    data.insert("search.button", "Искать");
    data.insert("search.results", "Результаты Поиска");
    data.insert("search.no_results", "Ничего не найдено");
    data.insert("search.try_different", "Попробуйте другой запрос");
    data.insert("search.tips", "Советы по Поиску");
    data.insert(
        "search.tip1",
        "Несколько слов ищут результаты со всеми терминами",
    );
    data.insert("search.tip2", "Поиск в заметках и Википедии");
    data.insert("search.tip3", "Совпадения в заголовке показываются первыми");

    // Wiki
    data.insert("wiki.search", "Поиск в Вики");
    data.insert("wiki.recent_articles", "Recent Articles");
    data.insert("wiki.manage", "Управление ZIM");
    data.insert("wiki.status", "Статус");
    data.insert("wiki.loaded", "загружено");
    data.insert("wiki.directory", "Директория ZIM");
    data.insert(
        "wiki.directory_hint",
        "Скопируйте файлы .zim в эту папку и обновите",
    );
    data.insert("wiki.refresh", "Обновить");
    data.insert("wiki.open_folder", "Открыть Папку");
    data.insert("wiki.add", "Добавить ZIM");
    data.insert("wiki.add_placeholder", "Введите путь к файлу ZIM...");
    data.insert("wiki.loaded_files", "Загруженные Файлы ZIM");
    data.insert("wiki.remove", "Удалить");
    data.insert("wiki.no_zim", "Нет файлов ZIM");
    data.insert(
        "wiki.no_zim_hint",
        "Добавьте файл ZIM для использования Википедии",
    );
    data.insert("wiki.no_zim_loaded", "Файлы ZIM не загружены");
    data.insert("wiki.loaded_zims", "Загруженные ZIM");
    data.insert("wiki.add_btn", "Добавить");
    data.insert("wiki.add_hint", "Введите полный путь к файлу ZIM.");
    data.insert("wiki.name", "Имя");
    data.insert("wiki.path", "Путь");
    data.insert("wiki.action", "Действие");
    data.insert("wiki.zim_added", " ZIM добавлено: ");
    data.insert("wiki.no_new_zim", "Нет новых файлов ZIM");
    data.insert("wiki.refresh_failed", "Ошибка обновления");
    data.insert("wiki.enter_path", "Пожалуйста введите путь");
    data.insert("wiki.add_failed", "Ошибка добавления");
    data.insert("wiki.remove_confirm", "Удалить этот ZIM?");
    data.insert("wiki.zim_removed", "ZIM удалён: ");
    data.insert("wiki.remove_failed", "Ошибка удаления");
    data.insert("wiki.open_folder_msg", "Откройте папку в проводнике:");

    // SRS
    data.insert("srs.title", "Повторение SRS");
    data.insert("srs.cards", "Список Карточек");
    data.insert("srs.streak", "подряд");
    data.insert("srs.show_answer", "Показать Ответ");
    data.insert("srs.again", "Снова");
    data.insert("srs.hard", "Сложно");
    data.insert("srs.good", "Хорошо");
    data.insert("srs.easy", "Легко");
    data.insert("srs.complete", "🎉 Сегодняшнее повторение завершено!");
    data.insert("srs.no_cards", "Нет карточек для повторения");
    data.insert("srs.progress", "карточек");
    data.insert("srs.today_review", "Сегодня");
    data.insert("srs.new_cards", "Новые");
    data.insert("srs.learning", "Изучение");
    data.insert("srs.mature", "Изучено");
    data.insert("srs.create_from_notes", "Создать карточки из заметок");
    data.insert("srs.create_cards", "Создать Карточки");
    data.insert("srs.cards_created", "карточек создано");
    data.insert("srs.no_cards_extracted", "Нет карточек для извлечения");
    data.insert("srs.repetitions", "Повторений");
    data.insert("srs.streak_days", "Подряд");
    data.insert("srs.start_review", "Начать Повторение");
    data.insert("srs.no_cards_yet", "Пока нет карточек.");
    data.insert(
        "srs.create_hint",
        "Откройте заметку и нажмите \"Создать Карточки\".",
    );
    data.insert("srs.interval", "Интервал");
    data.insert("srs.days", " дн.");
    data.insert("srs.delete_card_confirm", "Удалить эту карточку?");
    data.insert("srs.delete_failed", "Ошибка удаления");

    // Security
    data.insert("security.title", "Настройки Безопасности");
    data.insert("security.pin_not_set", "PIN не установлен");
    data.insert("security.pin_enabled", "PIN включён");
    data.insert("security.locked", "Заблокировано - Введите PIN");
    data.insert("security.pin_input", "Введите PIN (6-32 символов)");
    data.insert("security.set_pin", "Установить PIN");
    data.insert("security.remove_pin", "Удалить PIN");
    data.insert("security.lock", "Заблокировать");
    data.insert("security.unlock", "Разблокировать");
    data.insert("security.wrong_pin", "Неверный PIN");
    data.insert("security.pin_set_success", "PIN установлен");
    data.insert("security.pin_removed", "PIN удалён");
    data.insert("security.unlocked", "Разблокировано");
    data.insert(
        "security.pin_min_length",
        "PIN должен быть минимум 6 символов",
    );
    data.insert("security.enter_current_pin", "Введите текущий PIN");

    // Backup
    data.insert("backup.title", "Резервная Копия");
    data.insert("backup.info", "Информация о Копии");
    data.insert("backup.now", "Создать Копию");
    data.insert("backup.complete", "Копия создана");
    data.insert("backup.no_changes", "Нет изменений (копия пропущена)");

    // Split view
    data.insert("split.select_note", "Выберите заметку слева");

    // Common buttons
    data.insert("common.confirm", "Подтвердить");
    data.insert("common.cancel", "Отмена");
    data.insert("common.close", "Закрыть");
    data.insert("common.loading", "Загрузка...");
    data.insert("common.error", "Произошла ошибка");
    data.insert("common.success", "Успех");

    // Settings
    data.insert("settings.title", "Настройки");
    data.insert("settings.language", "Язык");
    data.insert("settings.language_desc", "Выберите предпочитаемый язык");
    data.insert("settings.stats", "Статистика");
    data.insert("settings.version", "Версия");
    data.insert("settings.storage", "Хранилище");
    data.insert("settings.about", "О программе");

    // Knowledge Graph & Footer
    data.insert("home.knowledge_graph", "Граф знаний");
    data.insert("app.footer", "Знания для всех");

    // Knowledge Graph & Footer
    data.insert("home.knowledge_graph", "Граф знаний");
    data.insert("app.footer", "Знания для всех");

    // === USB ===
    data.insert("usb.title", "Синхронизация USB");
    data.insert("usb.scan", "Сканировать");
    data.insert("usb.scanning", "Поиск USB устройств");
    data.insert("usb.no_usb", "Lazarus USB не обнаружен");
    data.insert(
        "usb.no_usb_hint",
        "Вставьте USB с lazarus.sync или инициализируйте ниже",
    );
    data.insert("usb.error", "Ошибка сканирования");
    data.insert("usb.init_title", "Инициализировать USB");
    data.insert(
        "usb.init_desc",
        "Создать новый Lazarus USB для офлайн обмена",
    );
    data.insert("usb.init_btn", "Инициализировать");
    data.insert("usb.init_error", "Ошибка инициализации");
    data.insert("usb.enter_path", "Введите путь USB");
    data.insert("usb.notes", "Заметки");
    data.insert("usb.posts", "Посты");
    data.insert("usb.packages", "Пакеты");
    data.insert("usb.sync", "Синхронизировать");
    data.insert("usb.export", "Экспорт");
    data.insert("usb.import", "Импорт");
    data.insert("home.usb_sync", "Синхронизация USB");
    data.insert("home.shortcut.wiki", "Вики");
    data.insert("home.shortcut.usb", "USB");
    data.insert("home.shortcut.graph", "Граф");
    data.insert("home.shortcut.review", "Повторение");
    Translations::new(data)
}
