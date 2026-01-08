//! العربية - Arabic translations (RTL)

use super::Translations;
use std::collections::HashMap;

pub fn translations() -> Translations {
    let mut data = HashMap::new();

    // Common
    data.insert("app.name", "لازاروس");
    data.insert("app.tagline", "إدارة المعرفة الشخصية بدون إنترنت");
    data.insert("nav.notes", "الملاحظات");
    data.insert("nav.search", "البحث");
    data.insert("nav.wiki", "ويكي");

    // Home
    data.insert("home.notes", "الملاحظات");
    data.insert("home.streak", "أيام متتالية");
    data.insert("home.day", "يوم");
    data.insert("home.quick_start", "بداية سريعة");
    data.insert("home.new_note", "ملاحظة جديدة");
    data.insert("home.note_list", "قائمة الملاحظات");
    data.insert("home.split_view", "عرض مقسم");
    data.insert("home.srs_review", "مراجعة SRS");
    data.insert("home.search", "البحث");
    data.insert("home.zim_manage", "إدارة ZIM");
    data.insert("home.security", "الأمان");
    data.insert("home.shortcuts", "اختصارات لوحة المفاتيح");
    data.insert("home.shortcut.new_note", "ملاحظة جديدة");
    data.insert("home.shortcut.search", "البحث");
    data.insert("home.shortcut.help", "المساعدة");

    // Editor
    data.insert("editor.title_placeholder", "أدخل العنوان");
    data.insert("editor.content_placeholder", "ابدأ الكتابة هنا...");
    data.insert("editor.tags", "الوسوم");
    data.insert("editor.tags_placeholder", "وسم1، وسم2، ...");
    data.insert("editor.edit_note", "تحرير الملاحظة");
    data.insert("editor.encrypt", "تبديل التشفير");
    data.insert("editor.focus", "تركيز");
    data.insert("editor.fullscreen", "ملء الشاشة");
    data.insert("editor.save", "حفظ");
    data.insert("editor.saved", "تم الحفظ");
    data.insert("editor.auto_saved", "حفظ تلقائي");
    data.insert("editor.changed", "تم التغيير...");
    data.insert("editor.words", "كلمات");
    data.insert("editor.save_complete", "💾 تم الحفظ!");
    data.insert("editor.encrypt_on", "🔒 التشفير مفعل");
    data.insert("editor.encrypt_off", "🔓 التشفير معطل");
    data.insert("editor.pin_required", "يرجى تعيين PIN أولاً (قائمة الأمان)");
    data.insert("editor.pin_locked", "يرجى فتح قفل PIN أولاً");

    // Notes list
    data.insert("notes.title", "الملاحظات");
    data.insert("notes.export", "تصدير");
    data.insert("notes.import", "استيراد");
    data.insert("notes.no_notes", "لا توجد ملاحظات");
    data.insert("notes.create_first", "أنشئ ملاحظتك الأولى!");
    data.insert("notes.no_title", "بدون عنوان");
    data.insert("notes.edit", "تحرير");
    data.insert("notes.delete", "حذف");
    data.insert("notes.delete_confirm", "هل أنت متأكد من الحذف؟");
    data.insert("notes.encrypted", "مشفر");
    data.insert("notes.created", "تم الإنشاء");
    data.insert("notes.updated", "تم التحديث");
    data.insert("notes.find_duplicates", "البحث عن المكررات");
    data.insert("notes.select_all", "تحديد الكل");
    data.insert("notes.selected", "محدد");
    data.insert("notes.create_package", "إنشاء حزمة");
    data.insert("notes.duplicates_title", "الملاحظات المكررة");
    data.insert("notes.no_duplicates", "لا توجد ملاحظات مكررة");
    data.insert("notes.export_package", "تصدير الحزمة");
    data.insert("notes.import_package", "استيراد الحزمة");
    data.insert("notes.package_title", "عنوان الحزمة");
    data.insert("notes.package_title_placeholder", "مثال: الفيزياء 101");
    data.insert("notes.package_author", "المؤلف");
    data.insert("notes.package_author_placeholder", "اسمك");
    data.insert("notes.package_description", "الوصف");
    data.insert(
        "notes.package_description_placeholder",
        "وصف الحزمة (اختياري)",
    );
    data.insert("notes.drop_file", "أسقط ملف .laz هنا");
    data.insert("notes.or", "أو");
    data.insert("notes.select_file", "اختر ملف");

    // Search
    data.insert("search.title", "البحث");
    data.insert("search.placeholder", "أدخل كلمة البحث...");
    data.insert("search.button", "بحث");
    data.insert("search.results", "نتائج البحث");
    data.insert("search.no_results", "لا توجد نتائج");
    data.insert("search.try_different", "جرب كلمة بحث مختلفة");
    data.insert("search.tips", "نصائح البحث");
    data.insert(
        "search.tip1",
        "كلمات متعددة تبحث عن نتائج تحتوي على جميع المصطلحات",
    );
    data.insert("search.tip2", "يبحث في الملاحظات وويكيبيديا");
    data.insert("search.tip3", "تظهر مطابقات العنوان أولاً");

    // Wiki
    data.insert("wiki.search", "بحث ويكي");
    data.insert("wiki.recent_articles", "Recent Articles");
    data.insert("wiki.manage", "إدارة ZIM");
    data.insert("wiki.status", "الحالة");
    data.insert("wiki.loaded", "تم التحميل");
    data.insert("wiki.directory", "مجلد ZIM");
    data.insert(
        "wiki.directory_hint",
        "انسخ ملفات .zim إلى هذا المجلد وقم بالتحديث",
    );
    data.insert("wiki.refresh", "تحديث");
    data.insert("wiki.open_folder", "فتح المجلد");
    data.insert("wiki.add", "إضافة ZIM");
    data.insert("wiki.add_placeholder", "أدخل مسار ملف ZIM...");
    data.insert("wiki.loaded_files", "ملفات ZIM المحملة");
    data.insert("wiki.remove", "إزالة");
    data.insert("wiki.no_zim", "لا توجد ملفات ZIM");
    data.insert("wiki.no_zim_hint", "أضف ملف ZIM لاستخدام ويكيبيديا");
    data.insert("wiki.no_zim_loaded", "لم يتم تحميل ملفات ZIM");
    data.insert("wiki.loaded_zims", "ملفات ZIM المحملة");
    data.insert("wiki.add_btn", "إضافة");
    data.insert("wiki.add_hint", "أدخل المسار الكامل لملف ZIM.");
    data.insert("wiki.name", "الاسم");
    data.insert("wiki.path", "المسار");
    data.insert("wiki.action", "إجراء");
    data.insert("wiki.zim_added", " تمت إضافة ZIM: ");
    data.insert("wiki.no_new_zim", "لا توجد ملفات ZIM جديدة");
    data.insert("wiki.refresh_failed", "فشل التحديث");
    data.insert("wiki.enter_path", "يرجى إدخال المسار");
    data.insert("wiki.add_failed", "فشلت الإضافة");
    data.insert("wiki.remove_confirm", "إزالة هذا ZIM؟");
    data.insert("wiki.zim_removed", "تمت إزالة ZIM: ");
    data.insert("wiki.remove_failed", "فشلت الإزالة");
    data.insert("wiki.open_folder_msg", "يرجى فتح المجلد في مستكشف الملفات:");

    // SRS
    data.insert("srs.title", "مراجعة SRS");
    data.insert("srs.cards", "قائمة البطاقات");
    data.insert("srs.streak", "متتالية");
    data.insert("srs.show_answer", "إظهار الإجابة");
    data.insert("srs.again", "مرة أخرى");
    data.insert("srs.hard", "صعب");
    data.insert("srs.good", "جيد");
    data.insert("srs.easy", "سهل");
    data.insert("srs.complete", "🎉 اكتملت مراجعة اليوم!");
    data.insert("srs.no_cards", "لا توجد بطاقات للمراجعة");
    data.insert("srs.progress", "بطاقات");
    data.insert("srs.today_review", "مراجعة اليوم");
    data.insert("srs.new_cards", "جديد");
    data.insert("srs.learning", "قيد التعلم");
    data.insert("srs.mature", "ناضج");
    data.insert("srs.create_from_notes", "إنشاء بطاقات من الملاحظات");
    data.insert("srs.create_cards", "إنشاء بطاقات");
    data.insert("srs.cards_created", "تم إنشاء البطاقات");
    data.insert("srs.no_cards_extracted", "لا توجد بطاقات للاستخراج");
    data.insert("srs.repetitions", "التكرارات");
    data.insert("srs.streak_days", "أيام متتالية");
    data.insert("srs.start_review", "بدء المراجعة");
    data.insert("srs.no_cards_yet", "لا توجد بطاقات بعد.");
    data.insert(
        "srs.create_hint",
        "افتح ملاحظة وانقر على زر \"إنشاء بطاقات\".",
    );
    data.insert("srs.interval", "الفاصل");
    data.insert("srs.days", " أيام");
    data.insert("srs.delete_card_confirm", "حذف هذه البطاقة؟");
    data.insert("srs.delete_failed", "فشل الحذف");

    // Security
    data.insert("security.title", "إعدادات الأمان");
    data.insert("security.pin_not_set", "لم يتم تعيين PIN");
    data.insert("security.pin_enabled", "PIN مفعل");
    data.insert("security.locked", "مقفل - أدخل PIN");
    data.insert("security.pin_input", "أدخل PIN (6-32 حرف)");
    data.insert("security.set_pin", "تعيين PIN");
    data.insert("security.remove_pin", "إزالة PIN");
    data.insert("security.lock", "قفل");
    data.insert("security.unlock", "فتح القفل");
    data.insert("security.wrong_pin", "PIN خاطئ");
    data.insert("security.pin_set_success", "تم تعيين PIN");
    data.insert("security.pin_removed", "تمت إزالة PIN");
    data.insert("security.unlocked", "تم فتح القفل");
    data.insert("security.pin_min_length", "يجب أن يكون PIN 6 أحرف على الأقل");
    data.insert("security.enter_current_pin", "أدخل PIN الحالي");

    // Backup
    data.insert("backup.title", "النسخ الاحتياطي");
    data.insert("backup.info", "معلومات النسخ الاحتياطي");
    data.insert("backup.now", "نسخ احتياطي الآن");
    data.insert("backup.complete", "اكتمل النسخ الاحتياطي");
    data.insert(
        "backup.no_changes",
        "لا توجد تغييرات (تم تخطي النسخ الاحتياطي)",
    );

    // Split view
    data.insert("split.select_note", "اختر ملاحظة من اليسار");

    // Common buttons
    data.insert("common.confirm", "تأكيد");
    data.insert("common.cancel", "إلغاء");
    data.insert("common.close", "إغلاق");
    data.insert("common.loading", "جاري التحميل...");
    data.insert("common.error", "حدث خطأ");
    data.insert("common.success", "نجاح");

    // Settings
    data.insert("settings.title", "الإعدادات");
    data.insert("settings.language", "اللغة");
    data.insert("settings.language_desc", "اختر لغتك المفضلة");
    data.insert("settings.stats", "الإحصائيات");
    data.insert("settings.version", "الإصدار");
    data.insert("settings.storage", "التخزين");
    data.insert("settings.about", "حول");

    // Knowledge Graph & Footer
    data.insert("home.knowledge_graph", "خريطة المعرفة");
    data.insert("app.footer", "المعرفة للجميع");

    // === USB ===
    data.insert("usb.title", "مزامنة USB");
    data.insert("usb.scan", "مسح");
    data.insert("usb.scanning", "جارٍ البحث عن أجهزة USB");
    data.insert("usb.no_usb", "لم يتم اكتشاف Lazarus USB");
    data.insert(
        "usb.no_usb_hint",
        "أدخل USB مع ملف lazarus.sync أو قم بالتهيئة أدناه",
    );
    data.insert("usb.error", "فشل المسح");
    data.insert("usb.init_title", "تهيئة USB");
    data.insert("usb.init_desc", "إنشاء Lazarus USB جديد للمشاركة دون اتصال");
    data.insert("usb.init_btn", "تهيئة");
    data.insert("usb.init_error", "فشل التهيئة");
    data.insert("usb.enter_path", "أدخل مسار USB");
    data.insert("usb.notes", "ملاحظات");
    data.insert("usb.posts", "منشورات");
    data.insert("usb.packages", "حزم");
    data.insert("usb.sync", "مزامنة");
    data.insert("usb.export", "تصدير");
    data.insert("usb.import", "استيراد");
    data.insert("home.usb_sync", "مزامنة USB");
    data.insert("home.shortcut.wiki", "ويكي");
    data.insert("home.shortcut.usb", "USB");
    data.insert("home.shortcut.graph", "رسم بياني");
    data.insert("home.shortcut.review", "مراجعة");

    // === Posts ===
    data.insert("posts.title", "المنشورات");
    data.insert("posts.new_post", "منشور جديد");
    data.insert("posts.no_posts", "لا توجد منشورات");
    data.insert("posts.be_first", "كن أول من ينشر!");
    data.insert("posts.author", "اسمك");
    data.insert("posts.title_placeholder", "العنوان");
    data.insert("posts.content_placeholder", "المحتوى...");
    data.insert("posts.tags_placeholder", "الوسوم (مفصولة بفواصل)");
    data.insert("posts.post_btn", "نشر");
    data.insert("posts.replies", "ردود");
    data.insert("posts.delete_confirm", "حذف هذا المنشور؟");
    data.insert("posts.write_reply", "اكتب رداً...");
    data.insert("posts.reply_btn", "رد");
    // === Q&A ===
    data.insert("qna.title", "أسئلة وأجوبة");
    data.insert("qna.ask_question", "اطرح سؤالاً");
    data.insert("qna.no_questions", "لا توجد أسئلة");
    data.insert("qna.be_first", "كن أول من يسأل!");
    data.insert("qna.question_title", "عنوان السؤال");
    data.insert("qna.question_content", "اشرح سؤالك...");
    data.insert("qna.post_question", "نشر السؤال");
    data.insert("qna.answers", "إجابات");
    data.insert("qna.write_answer", "اكتب إجابتك...");
    data.insert("qna.post_answer", "نشر الإجابة");
    data.insert("qna.accept", "قبول");
    data.insert("qna.accepted", "مقبول");
    data.insert("qna.delete_confirm", "حذف هذا السؤال؟");

    Translations::new(data)
}
