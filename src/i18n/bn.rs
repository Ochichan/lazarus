//! Bengali (বাংলা) translations

use super::Translations;
use std::collections::HashMap;

pub fn translations() -> Translations {
    let mut data = HashMap::new();

    // Common
    data.insert("app.name", "Lazarus");
    data.insert("app.tagline", "অফলাইন ব্যক্তিগত জ্ঞান ব্যবস্থাপনা");
    data.insert("nav.notes", "নোট");
    data.insert("nav.search", "অনুসন্ধান");
    data.insert("nav.wiki", "উইকি");

    // Home
    data.insert("home.notes", "নোট");
    data.insert("home.streak", "অধ্যয়ন ধারা");
    data.insert("home.day", "দিন");
    data.insert("home.quick_start", "দ্রুত শুরু");
    data.insert("home.new_note", "নতুন নোট");
    data.insert("home.note_list", "নোট তালিকা");
    data.insert("home.split_view", "বিভক্ত দৃশ্য");
    data.insert("home.srs_review", "SRS পর্যালোচনা");
    data.insert("home.search", "অনুসন্ধান");
    data.insert("home.zim_manage", "ZIM ব্যবস্থাপনা");
    data.insert("home.security", "নিরাপত্তা");
    data.insert("home.shortcuts", "কীবোর্ড শর্টকাট");
    data.insert("home.shortcut.new_note", "নতুন নোট");
    data.insert("home.shortcut.search", "অনুসন্ধান");
    data.insert("home.shortcut.help", "সাহায্য");

    // Editor
    data.insert("editor.title_placeholder", "শিরোনাম লিখুন");
    data.insert("editor.content_placeholder", "এখানে লেখা শুরু করুন...");
    data.insert("editor.tags", "ট্যাগ");
    data.insert("editor.tags_placeholder", "ট্যাগ১, ট্যাগ২, ...");
    data.insert("editor.edit_note", "নোট সম্পাদনা");
    data.insert("editor.encrypt", "এনক্রিপশন টগল");
    data.insert("editor.focus", "ফোকাস");
    data.insert("editor.fullscreen", "পূর্ণ স্ক্রিন");
    data.insert("editor.save", "সংরক্ষণ");
    data.insert("editor.saved", "সংরক্ষিত");
    data.insert("editor.auto_saved", "স্বয়ংক্রিয় সংরক্ষিত");
    data.insert("editor.changed", "পরিবর্তিত...");
    data.insert("editor.words", "শব্দ");
    data.insert("editor.save_complete", "💾 সংরক্ষিত!");
    data.insert("editor.encrypt_on", "🔒 এনক্রিপশন সক্রিয়");
    data.insert("editor.encrypt_off", "🔓 এনক্রিপশন নিষ্ক্রিয়");
    data.insert("editor.pin_required", "প্রথমে PIN সেট করুন (নিরাপত্তা মেনু)");
    data.insert("editor.pin_locked", "প্রথমে PIN আনলক করুন");

    // Notes list
    data.insert("notes.title", "নোট");
    data.insert("notes.export", "রপ্তানি");
    data.insert("notes.import", "আমদানি");
    data.insert("notes.no_notes", "এখনো কোনো নোট নেই");
    data.insert("notes.create_first", "আপনার প্রথম নোট তৈরি করুন!");
    data.insert("notes.no_title", "শিরোনামহীন");
    data.insert("notes.edit", "সম্পাদনা");
    data.insert("notes.delete", "মুছুন");
    data.insert("notes.delete_confirm", "আপনি কি নিশ্চিত মুছতে চান?");
    data.insert("notes.encrypted", "এনক্রিপ্টেড");
    data.insert("notes.created", "তৈরি");
    data.insert("notes.updated", "আপডেট");
    data.insert("notes.find_duplicates", "সদৃশ খুঁজুন");
    data.insert("notes.select_all", "সব নির্বাচন");
    data.insert("notes.selected", "নির্বাচিত");
    data.insert("notes.create_package", "প্যাকেজ তৈরি");
    data.insert("notes.duplicates_title", "সদৃশ নোট");
    data.insert("notes.no_duplicates", "কোনো সদৃশ নোট পাওয়া যায়নি");
    data.insert("notes.export_package", "প্যাকেজ রপ্তানি");
    data.insert("notes.import_package", "প্যাকেজ আমদানি");
    data.insert("notes.package_title", "প্যাকেজ শিরোনাম");
    data.insert("notes.package_title_placeholder", "যেমন, পদার্থবিদ্যা ১০১");
    data.insert("notes.package_author", "লেখক");
    data.insert("notes.package_author_placeholder", "আপনার নাম");
    data.insert("notes.package_description", "বিবরণ");
    data.insert(
        "notes.package_description_placeholder",
        "প্যাকেজ বিবরণ (ঐচ্ছিক)",
    );
    data.insert("notes.drop_file", ".laz ফাইল এখানে ড্রপ করুন");
    data.insert("notes.or", "অথবা");
    data.insert("notes.select_file", "ফাইল নির্বাচন");

    // Search
    data.insert("search.title", "অনুসন্ধান");
    data.insert("search.placeholder", "অনুসন্ধান শব্দ লিখুন...");
    data.insert("search.button", "অনুসন্ধান");
    data.insert("search.results", "অনুসন্ধান ফলাফল");
    data.insert("search.no_results", "কোনো ফলাফল পাওয়া যায়নি");
    data.insert("search.try_different", "অন্য শব্দ দিয়ে চেষ্টা করুন");
    data.insert("search.tips", "অনুসন্ধান টিপস");
    data.insert("search.tip1", "একাধিক শব্দ সব শব্দ সহ ফলাফল খোঁজে");
    data.insert("search.tip2", "নোট এবং উইকিপিডিয়া উভয়ে অনুসন্ধান করে");
    data.insert("search.tip3", "শিরোনাম মিল প্রথমে দেখায়");

    // Wiki
    data.insert("wiki.search", "উইকি অনুসন্ধান");
    data.insert("wiki.recent_articles", "Recent Articles");
    data.insert("wiki.manage", "ZIM ব্যবস্থাপনা");
    data.insert("wiki.status", "অবস্থা");
    data.insert("wiki.loaded", "লোড হয়েছে");
    data.insert("wiki.directory", "ZIM ডিরেক্টরি");
    data.insert(
        "wiki.directory_hint",
        "এই ফোল্ডারে .zim ফাইল কপি করে রিফ্রেশ করুন",
    );
    data.insert("wiki.refresh", "রিফ্রেশ");
    data.insert("wiki.open_folder", "ফোল্ডার খুলুন");
    data.insert("wiki.add", "ZIM যোগ করুন");
    data.insert("wiki.add_placeholder", "ZIM ফাইল পাথ লিখুন...");
    data.insert("wiki.loaded_files", "লোড হওয়া ZIM ফাইল");
    data.insert("wiki.remove", "সরান");
    data.insert("wiki.no_zim", "কোনো ZIM ফাইল নেই");
    data.insert("wiki.no_zim_hint", "উইকিপিডিয়া ব্যবহার করতে ZIM ফাইল যোগ করুন");
    data.insert("wiki.no_zim_loaded", "কোনো ZIM ফাইল লোড হয়নি");
    data.insert("wiki.loaded_zims", "লোড হওয়া ZIM");
    data.insert("wiki.add_btn", "যোগ করুন");
    data.insert("wiki.add_hint", "ZIM ফাইলের সম্পূর্ণ পাথ লিখুন।");
    data.insert("wiki.name", "নাম");
    data.insert("wiki.path", "পাথ");
    data.insert("wiki.action", "অ্যাকশন");
    data.insert("wiki.zim_added", " ZIM যোগ হয়েছে: ");
    data.insert("wiki.no_new_zim", "কোনো নতুন ZIM ফাইল নেই");
    data.insert("wiki.refresh_failed", "রিফ্রেশ ব্যর্থ");
    data.insert("wiki.enter_path", "পাথ লিখুন");
    data.insert("wiki.add_failed", "যোগ করা ব্যর্থ");
    data.insert("wiki.remove_confirm", "এই ZIM সরাতে চান?");
    data.insert("wiki.zim_removed", "ZIM সরানো হয়েছে: ");
    data.insert("wiki.remove_failed", "সরানো ব্যর্থ");
    data.insert("wiki.open_folder_msg", "ফাইল এক্সপ্লোরারে ফোল্ডারটি খুলুন:");

    // SRS
    data.insert("srs.title", "SRS পর্যালোচনা");
    data.insert("srs.cards", "কার্ড তালিকা");
    data.insert("srs.streak", "ধারা");
    data.insert("srs.show_answer", "উত্তর দেখান");
    data.insert("srs.again", "আবার");
    data.insert("srs.hard", "কঠিন");
    data.insert("srs.good", "ভালো");
    data.insert("srs.easy", "সহজ");
    data.insert("srs.complete", "🎉 আজকের পর্যালোচনা সম্পন্ন!");
    data.insert("srs.no_cards", "পর্যালোচনার জন্য কোনো কার্ড নেই");
    data.insert("srs.progress", "কার্ড");
    data.insert("srs.today_review", "আজ বাকি");
    data.insert("srs.new_cards", "নতুন");
    data.insert("srs.learning", "শিখছি");
    data.insert("srs.mature", "পরিপক্ক");
    data.insert("srs.create_from_notes", "নোট থেকে কার্ড তৈরি করুন");
    data.insert("srs.create_cards", "কার্ড তৈরি");
    data.insert("srs.cards_created", "কার্ড তৈরি হয়েছে");
    data.insert("srs.no_cards_extracted", "কোনো কার্ড বের করা যায়নি");
    data.insert("srs.repetitions", "পুনরাবৃত্তি");
    data.insert("srs.streak_days", "ধারা");
    data.insert("srs.start_review", "পর্যালোচনা শুরু");
    data.insert("srs.no_cards_yet", "এখনো কোনো কার্ড নেই।");
    data.insert(
        "srs.create_hint",
        "একটি নোট খুলুন এবং \"কার্ড তৈরি\" বোতাম ক্লিক করুন।",
    );
    data.insert("srs.interval", "ব্যবধান");
    data.insert("srs.days", " দিন");
    data.insert("srs.delete_card_confirm", "এই কার্ড মুছবেন?");
    data.insert("srs.delete_failed", "মুছতে ব্যর্থ");

    // Security
    data.insert("security.title", "নিরাপত্তা সেটিংস");
    data.insert("security.pin_not_set", "PIN সেট নেই");
    data.insert("security.pin_enabled", "PIN সক্রিয়");
    data.insert("security.locked", "লক - PIN দিন");
    data.insert("security.pin_input", "PIN দিন (৬-৩২ অক্ষর)");
    data.insert("security.set_pin", "PIN সেট করুন");
    data.insert("security.remove_pin", "PIN সরান");
    data.insert("security.lock", "লক");
    data.insert("security.unlock", "আনলক");
    data.insert("security.wrong_pin", "ভুল PIN");
    data.insert("security.pin_set_success", "PIN সেট হয়েছে");
    data.insert("security.pin_removed", "PIN সরানো হয়েছে");
    data.insert("security.unlocked", "আনলক হয়েছে");
    data.insert("security.pin_min_length", "PIN কমপক্ষে ৬ অক্ষর হতে হবে");
    data.insert("security.enter_current_pin", "বর্তমান PIN দিন");

    // Backup
    data.insert("backup.title", "ব্যাকআপ");
    data.insert("backup.info", "ব্যাকআপ তথ্য");
    data.insert("backup.now", "এখনই ব্যাকআপ");
    data.insert("backup.complete", "ব্যাকআপ সম্পন্ন");
    data.insert("backup.no_changes", "কোনো পরিবর্তন নেই (ব্যাকআপ এড়ানো হয়েছে)");

    // Split view
    data.insert("split.select_note", "বাম থেকে একটি নোট নির্বাচন করুন");

    // Common buttons
    data.insert("common.confirm", "নিশ্চিত");
    data.insert("common.cancel", "বাতিল");
    data.insert("common.close", "বন্ধ");
    data.insert("common.loading", "লোড হচ্ছে...");
    data.insert("common.error", "একটি ত্রুটি ঘটেছে");
    data.insert("common.success", "সফল");

    // Settings
    data.insert("settings.title", "সেটিংস");
    data.insert("settings.language", "ভাষা");
    data.insert("settings.language_desc", "আপনার পছন্দের ভাষা নির্বাচন করুন");
    data.insert("settings.stats", "পরিসংখ্যান");
    data.insert("settings.version", "সংস্করণ");
    data.insert("settings.storage", "স্টোরেজ");
    data.insert("settings.about", "সম্পর্কে");

    // Knowledge Graph & Footer
    data.insert("home.knowledge_graph", "জ্ঞান গ্রাফ");
    data.insert("app.footer", "সবার জন্য জ্ঞান");

    // === USB ===
    data.insert("usb.title", "USB সিঙ্ক");
    data.insert("usb.scan", "স্ক্যান");
    data.insert("usb.scanning", "USB ডিভাইস খোঁজা হচ্ছে");
    data.insert("usb.no_usb", "কোনো Lazarus USB পাওয়া যায়নি");
    data.insert(
        "usb.no_usb_hint",
        "lazarus.sync সহ USB ঢোকান বা নিচে ইনিশিয়ালাইজ করুন",
    );
    data.insert("usb.error", "স্ক্যান ব্যর্থ");
    data.insert("usb.init_title", "USB ইনিশিয়ালাইজ");
    data.insert(
        "usb.init_desc",
        "অফলাইন শেয়ারিং এর জন্য নতুন Lazarus USB তৈরি করুন",
    );
    data.insert("usb.init_btn", "ইনিশিয়ালাইজ");
    data.insert("usb.init_error", "ইনিশিয়ালাইজ ব্যর্থ");
    data.insert("usb.enter_path", "USB পাথ দিন");
    data.insert("usb.notes", "নোট");
    data.insert("usb.posts", "পোস্ট");
    data.insert("usb.packages", "প্যাকেজ");
    data.insert("usb.sync", "সিঙ্ক");
    data.insert("usb.export", "এক্সপোর্ট");
    data.insert("usb.import", "ইমপোর্ট");
    data.insert("home.usb_sync", "USB সিঙ্ক");
    data.insert("home.shortcut.wiki", "উইকি");
    data.insert("home.shortcut.usb", "USB");
    data.insert("home.shortcut.graph", "গ্রাফ");
    data.insert("home.shortcut.review", "রিভিউ");
    Translations::new(data)
}
