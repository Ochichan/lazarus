//! Persian (فارسی) translations - RTL

use super::Translations;
use std::collections::HashMap;

pub fn translations() -> Translations {
    let mut data = HashMap::new();

    // Common
    data.insert("app.name", "Lazarus");
    data.insert("app.tagline", "مدیریت دانش شخصی آفلاین");
    data.insert("nav.notes", "یادداشت‌ها");
    data.insert("nav.search", "جستجو");
    data.insert("nav.wiki", "ویکی");

    // Home
    data.insert("home.notes", "یادداشت‌ها");
    data.insert("home.streak", "روند مطالعه");
    data.insert("home.day", "روز");
    data.insert("home.quick_start", "شروع سریع");
    data.insert("home.new_note", "یادداشت جدید");
    data.insert("home.note_list", "لیست یادداشت‌ها");
    data.insert("home.split_view", "نمای دوبخشی");
    data.insert("home.srs_review", "مرور SRS");
    data.insert("home.search", "جستجو");
    data.insert("home.zim_manage", "مدیریت ZIM");
    data.insert("home.security", "امنیت");
    data.insert("home.shortcuts", "میانبرهای صفحه‌کلید");
    data.insert("home.shortcut.new_note", "یادداشت جدید");
    data.insert("home.shortcut.search", "جستجو");
    data.insert("home.shortcut.help", "راهنما");

    // Editor
    data.insert("editor.title_placeholder", "عنوان را وارد کنید");
    data.insert("editor.content_placeholder", "اینجا بنویسید...");
    data.insert("editor.tags", "برچسب‌ها");
    data.insert("editor.tags_placeholder", "برچسب۱، برچسب۲، ...");
    data.insert("editor.edit_note", "ویرایش یادداشت");
    data.insert("editor.encrypt", "تغییر رمزنگاری");
    data.insert("editor.focus", "تمرکز");
    data.insert("editor.fullscreen", "تمام‌صفحه");
    data.insert("editor.save", "ذخیره");
    data.insert("editor.saved", "ذخیره شد");
    data.insert("editor.auto_saved", "خودکار ذخیره شد");
    data.insert("editor.changed", "تغییر کرد...");
    data.insert("editor.words", "کلمه");
    data.insert("editor.save_complete", "💾 ذخیره شد!");
    data.insert("editor.encrypt_on", "🔒 رمزنگاری فعال");
    data.insert("editor.encrypt_off", "🔓 رمزنگاری غیرفعال");
    data.insert("editor.pin_required", "ابتدا پین تنظیم کنید (منوی امنیت)");
    data.insert("editor.pin_locked", "ابتدا پین را باز کنید");

    // Notes list
    data.insert("notes.title", "یادداشت‌ها");
    data.insert("notes.export", "خروجی");
    data.insert("notes.import", "ورودی");
    data.insert("notes.no_notes", "هنوز یادداشتی نیست");
    data.insert("notes.create_first", "اولین یادداشت خود را بسازید!");
    data.insert("notes.no_title", "بدون عنوان");
    data.insert("notes.edit", "ویرایش");
    data.insert("notes.delete", "حذف");
    data.insert(
        "notes.delete_confirm",
        "آیا مطمئن هستید که می‌خواهید حذف کنید؟",
    );
    data.insert("notes.encrypted", "رمزنگاری شده");
    data.insert("notes.created", "ایجاد شده");
    data.insert("notes.updated", "به‌روزرسانی شده");
    data.insert("notes.find_duplicates", "یافتن تکراری‌ها");
    data.insert("notes.select_all", "انتخاب همه");
    data.insert("notes.selected", "انتخاب شده");
    data.insert("notes.create_package", "ایجاد بسته");
    data.insert("notes.duplicates_title", "یادداشت‌های تکراری");
    data.insert("notes.no_duplicates", "یادداشت تکراری یافت نشد");
    data.insert("notes.export_package", "خروجی بسته");
    data.insert("notes.import_package", "ورودی بسته");
    data.insert("notes.package_title", "عنوان بسته");
    data.insert("notes.package_title_placeholder", "مثلاً فیزیک ۱۰۱");
    data.insert("notes.package_author", "نویسنده");
    data.insert("notes.package_author_placeholder", "نام شما");
    data.insert("notes.package_description", "توضیحات");
    data.insert(
        "notes.package_description_placeholder",
        "توضیحات بسته (اختیاری)",
    );
    data.insert("notes.drop_file", "فایل .laz را اینجا رها کنید");
    data.insert("notes.or", "یا");
    data.insert("notes.select_file", "انتخاب فایل");

    // Search
    data.insert("search.title", "جستجو");
    data.insert("search.placeholder", "عبارت جستجو را وارد کنید...");
    data.insert("search.button", "جستجو");
    data.insert("search.results", "نتایج جستجو");
    data.insert("search.no_results", "نتیجه‌ای یافت نشد");
    data.insert("search.try_different", "عبارت دیگری امتحان کنید");
    data.insert("search.tips", "راهنمای جستجو");
    data.insert(
        "search.tip1",
        "چند کلمه نتایج شامل همه عبارات را جستجو می‌کند",
    );
    data.insert("search.tip2", "در یادداشت‌ها و ویکی‌پدیا جستجو می‌کند");
    data.insert("search.tip3", "تطابق عنوان ابتدا نمایش داده می‌شود");

    // Wiki
    data.insert("wiki.search", "جستجوی ویکی");
    data.insert("wiki.manage", "مدیریت ZIM");
    data.insert("wiki.status", "وضعیت");
    data.insert("wiki.loaded", "بارگذاری شده");
    data.insert("wiki.directory", "دایرکتوری ZIM");
    data.insert(
        "wiki.directory_hint",
        "فایل‌های .zim را در این پوشه کپی کنید و تازه‌سازی کنید",
    );
    data.insert("wiki.refresh", "تازه‌سازی");
    data.insert("wiki.open_folder", "باز کردن پوشه");
    data.insert("wiki.add", "افزودن ZIM");
    data.insert("wiki.add_placeholder", "مسیر فایل ZIM را وارد کنید...");
    data.insert("wiki.loaded_files", "فایل‌های ZIM بارگذاری شده");
    data.insert("wiki.remove", "حذف");
    data.insert("wiki.no_zim", "فایل ZIM موجود نیست");
    data.insert(
        "wiki.no_zim_hint",
        "برای استفاده از ویکی‌پدیا فایل ZIM اضافه کنید",
    );
    data.insert("wiki.no_zim_loaded", "فایل ZIM بارگذاری نشده");
    data.insert("wiki.loaded_zims", "ZIMهای بارگذاری شده");
    data.insert("wiki.add_btn", "افزودن");
    data.insert("wiki.add_hint", "مسیر کامل فایل ZIM را وارد کنید.");
    data.insert("wiki.name", "نام");
    data.insert("wiki.path", "مسیر");
    data.insert("wiki.action", "عمل");
    data.insert("wiki.zim_added", " ZIM اضافه شد: ");
    data.insert("wiki.no_new_zim", "فایل ZIM جدیدی نیست");
    data.insert("wiki.refresh_failed", "تازه‌سازی ناموفق");
    data.insert("wiki.enter_path", "مسیر را وارد کنید");
    data.insert("wiki.add_failed", "افزودن ناموفق");
    data.insert("wiki.remove_confirm", "این ZIM حذف شود؟");
    data.insert("wiki.zim_removed", "ZIM حذف شد: ");
    data.insert("wiki.remove_failed", "حذف ناموفق");
    data.insert("wiki.open_folder_msg", "پوشه را در مدیر فایل باز کنید:");

    // SRS
    data.insert("srs.title", "مرور SRS");
    data.insert("srs.cards", "لیست کارت‌ها");
    data.insert("srs.streak", "روند");
    data.insert("srs.show_answer", "نمایش پاسخ");
    data.insert("srs.again", "دوباره");
    data.insert("srs.hard", "سخت");
    data.insert("srs.good", "خوب");
    data.insert("srs.easy", "آسان");
    data.insert("srs.complete", "🎉 مرور امروز تمام شد!");
    data.insert("srs.no_cards", "کارتی برای مرور نیست");
    data.insert("srs.progress", "کارت");
    data.insert("srs.today_review", "امروز باقی‌مانده");
    data.insert("srs.new_cards", "جدید");
    data.insert("srs.learning", "در حال یادگیری");
    data.insert("srs.mature", "بالغ");
    data.insert("srs.create_from_notes", "ایجاد کارت از یادداشت‌ها");
    data.insert("srs.create_cards", "ایجاد کارت");
    data.insert("srs.cards_created", "کارت ایجاد شد");
    data.insert("srs.no_cards_extracted", "کارتی استخراج نشد");
    data.insert("srs.repetitions", "تکرار");
    data.insert("srs.streak_days", "روند");
    data.insert("srs.start_review", "شروع مرور");
    data.insert("srs.no_cards_yet", "هنوز کارتی نیست.");
    data.insert(
        "srs.create_hint",
        "یادداشتی باز کنید و دکمه \"ایجاد کارت\" را بزنید.",
    );
    data.insert("srs.interval", "فاصله");
    data.insert("srs.days", " روز");
    data.insert("srs.delete_card_confirm", "این کارت حذف شود؟");
    data.insert("srs.delete_failed", "حذف ناموفق");

    // Security
    data.insert("security.title", "تنظیمات امنیت");
    data.insert("security.pin_not_set", "پین تنظیم نشده");
    data.insert("security.pin_enabled", "پین فعال");
    data.insert("security.locked", "قفل - پین وارد کنید");
    data.insert("security.pin_input", "پین وارد کنید (۶-۳۲ کاراکتر)");
    data.insert("security.set_pin", "تنظیم پین");
    data.insert("security.remove_pin", "حذف پین");
    data.insert("security.lock", "قفل");
    data.insert("security.unlock", "باز کردن قفل");
    data.insert("security.wrong_pin", "پین اشتباه");
    data.insert("security.pin_set_success", "پین تنظیم شد");
    data.insert("security.pin_removed", "پین حذف شد");
    data.insert("security.unlocked", "قفل باز شد");
    data.insert("security.pin_min_length", "پین باید حداقل ۶ کاراکتر باشد");
    data.insert("security.enter_current_pin", "پین فعلی را وارد کنید");

    // Backup
    data.insert("backup.title", "پشتیبان‌گیری");
    data.insert("backup.info", "اطلاعات پشتیبان");
    data.insert("backup.now", "پشتیبان‌گیری الان");
    data.insert("backup.complete", "پشتیبان‌گیری کامل شد");
    data.insert("backup.no_changes", "تغییری نیست (پشتیبان‌گیری رد شد)");

    // Split view
    data.insert("split.select_note", "یادداشتی از سمت چپ انتخاب کنید");

    // Common buttons
    data.insert("common.confirm", "تأیید");
    data.insert("common.cancel", "لغو");
    data.insert("common.close", "بستن");
    data.insert("common.loading", "در حال بارگذاری...");
    data.insert("common.error", "خطایی رخ داد");
    data.insert("common.success", "موفق");

    // Settings
    data.insert("settings.title", "تنظیمات");
    data.insert("settings.language", "زبان");
    data.insert("settings.language_desc", "زبان مورد نظر خود را انتخاب کنید");
    data.insert("settings.stats", "آمار");
    data.insert("settings.version", "نسخه");
    data.insert("settings.storage", "ذخیره‌سازی");
    data.insert("settings.about", "درباره");

    // Knowledge Graph & Footer
    data.insert("home.knowledge_graph", "نمودار دانش");
    data.insert("app.footer", "دانش برای همه");

    Translations::new(data)
}
