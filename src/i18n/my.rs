//! Burmese (Myanmar) translations

use super::Translations;
use std::collections::HashMap;

pub fn translations() -> Translations {
    let mut data = HashMap::new();

    // Common
    data.insert("app.name", "Lazarus");
    data.insert("app.tagline", "အော့ဖ်လိုင်း ပုဂ္ဂိုလ်ရေး အသိပညာ စီမံခန့်ခွဲမှု");
    data.insert("nav.notes", "မှတ်စု");
    data.insert("nav.search", "ရှာဖွေ");
    data.insert("nav.wiki", "ဝီကီ");

    // Home
    data.insert("home.notes", "မှတ်စု");
    data.insert("home.streak", "လေ့လာမှု ဆက်တိုက်");
    data.insert("home.day", "ရက်");
    data.insert("home.quick_start", "မြန်မြန် စတင်ရန်");
    data.insert("home.new_note", "မှတ်စု အသစ်");
    data.insert("home.note_list", "မှတ်စု စာရင်း");
    data.insert("home.split_view", "ခွဲထားသော မြင်ကွင်း");
    data.insert("home.srs_review", "SRS ပြန်လည်သုံးသပ်");
    data.insert("home.search", "ရှာဖွေ");
    data.insert("home.zim_manage", "ZIM စီမံခန့်ခွဲ");
    data.insert("home.security", "လုံခြုံရေး");
    data.insert("home.shortcuts", "ကီးဘုတ် ဖြတ်လမ်း");
    data.insert("home.shortcut.new_note", "မှတ်စု အသစ်");
    data.insert("home.shortcut.search", "ရှာဖွေ");
    data.insert("home.shortcut.help", "အကူအညီ");
    data.insert("home.shortcut.wiki", "ဝီကီ");
    data.insert("home.shortcut.usb", "USB");
    data.insert("home.shortcut.graph", "ဂရပ်ဖ်");
    data.insert("home.shortcut.review", "ပြန်လည်သုံးသပ်");
    data.insert("home.knowledge_graph", "အသိပညာ ဂရပ်ဖ်");
    data.insert("home.usb_sync", "USB စင့်ခ်");
    data.insert("app.footer", "အားလုံးအတွက် အသိပညာ");

    // Editor
    data.insert("editor.title_placeholder", "ခေါင်းစဉ် ထည့်ပါ");
    data.insert("editor.content_placeholder", "ဤနေရာတွင် စတင်ရေးပါ...");
    data.insert("editor.tags", "တဂ်များ");
    data.insert("editor.tags_placeholder", "တဂ်၁၊ တဂ်၂၊ ...");
    data.insert("editor.edit_note", "မှတ်စု တည်းဖြတ်");
    data.insert("editor.encrypt", "စာဝှက် ဖွင့်/ပိတ်");
    data.insert("editor.focus", "အာရုံစိုက်");
    data.insert("editor.fullscreen", "မျက်နှာပြင်အပြည့်");
    data.insert("editor.save", "သိမ်းဆည်း");
    data.insert("editor.saved", "သိမ်းပြီး");
    data.insert("editor.auto_saved", "အလိုအလျောက် သိမ်းပြီး");
    data.insert("editor.changed", "ပြောင်းလဲနေ...");
    data.insert("editor.words", "စကားလုံး");
    data.insert("editor.save_complete", "💾 သိမ်းပြီး!");
    data.insert("editor.encrypt_on", "🔒 စာဝှက် ဖွင့်ထား");
    data.insert("editor.encrypt_off", "🔓 စာဝှက် ပိတ်ထား");
    data.insert("editor.pin_required", "PIN ဦးစွာ သတ်မှတ်ပါ (လုံခြုံရေး မီနူး)");
    data.insert("editor.pin_locked", "PIN ဦးစွာ ဖွင့်ပါ");

    // Notes list
    data.insert("notes.title", "မှတ်စုများ");
    data.insert("notes.export", "ထုတ်ယူ");
    data.insert("notes.import", "သွင်းယူ");
    data.insert("notes.no_notes", "မှတ်စု မရှိသေး");
    data.insert("notes.create_first", "ပထမ မှတ်စု ဖန်တီးပါ!");
    data.insert("notes.no_title", "ခေါင်းစဉ်မဲ့");
    data.insert("notes.edit", "တည်းဖြတ်");
    data.insert("notes.delete", "ဖျက်");
    data.insert("notes.delete_confirm", "ဖျက်မည် သေချာပါသလား?");
    data.insert("notes.encrypted", "စာဝှက်ထား");
    data.insert("notes.created", "ဖန်တီးသည်");
    data.insert("notes.updated", "ပြင်ဆင်သည်");
    data.insert("notes.find_duplicates", "ထပ်တူ ရှာဖွေ");
    data.insert("notes.select_all", "အားလုံး ရွေး");
    data.insert("notes.selected", "ရွေးထား");
    data.insert("notes.create_package", "Package ဖန်တီး");
    data.insert("notes.duplicates_title", "ထပ်တူ မှတ်စုများ");
    data.insert("notes.no_duplicates", "ထပ်တူ မရှိ");
    data.insert("notes.export_package", "Package ထုတ်ယူ");
    data.insert("notes.import_package", "Package သွင်းယူ");
    data.insert("notes.package_title", "Package ခေါင်းစဉ်");
    data.insert("notes.package_title_placeholder", "ဥပမာ - ရူပဗေဒ ၁၀၁");
    data.insert("notes.package_author", "ရေးသူ");
    data.insert("notes.package_author_placeholder", "သင့်အမည်");
    data.insert("notes.package_description", "ဖော်ပြချက်");
    data.insert(
        "notes.package_description_placeholder",
        "Package ဖော်ပြချက် (ရွေးချယ်မှု)",
    );
    data.insert("notes.drop_file", ".laz ဖိုင် ဤနေရာ ချပါ");
    data.insert("notes.or", "သို့မဟုတ်");
    data.insert("notes.select_file", "ဖိုင် ရွေးချယ်");

    // Search
    data.insert("search.title", "ရှာဖွေ");
    data.insert("search.placeholder", "ရှာဖွေရန် စာသား ထည့်ပါ...");
    data.insert("search.button", "ရှာဖွေ");
    data.insert("search.results", "ရှာဖွေမှု ရလဒ်များ");
    data.insert("search.no_results", "ရလဒ် မတွေ့ပါ");
    data.insert("search.try_different", "အခြား စာသား စမ်းကြည့်ပါ");
    data.insert("search.tips", "ရှာဖွေမှု အကြံပြု");
    data.insert("search.tip1", "စကားလုံးများစွာ အားလုံးပါဝင်သော ရလဒ်များ ရှာပေးသည်");
    data.insert("search.tip2", "မှတ်စုနှင့် ဝီကီပီးဒီးယား နှစ်ခုလုံး ရှာသည်");
    data.insert("search.tip3", "ခေါင်းစဉ် တူညီမှု ဦးစားပေး");

    // Wiki
    data.insert("wiki.search", "ဝီကီ ရှာဖွေ");
    data.insert("wiki.recent_articles", "မကြာသေးမီ ဆောင်းပါးများ");
    data.insert("wiki.manage", "ZIM စီမံခန့်ခွဲ");
    data.insert("wiki.status", "အခြေအနေ");
    data.insert("wiki.loaded", "တင်ထား");
    data.insert("wiki.directory", "ZIM ဖိုဒါ");
    data.insert("wiki.directory_hint", ".zim ဖိုင်များကို ဤဖိုဒါသို့ ကူးပြီး ပြန်လည်စတင်ပါ");
    data.insert("wiki.refresh", "ပြန်လည်စတင်");
    data.insert("wiki.open_folder", "ဖိုဒါ ဖွင့်");
    data.insert("wiki.add", "ZIM ထည့်");
    data.insert("wiki.add_placeholder", "ZIM ဖိုင် လမ်းကြောင်း ထည့်ပါ...");
    data.insert("wiki.loaded_files", "တင်ထားသော ZIM ဖိုင်များ");
    data.insert("wiki.remove", "ဖယ်ရှား");
    data.insert("wiki.no_zim", "ZIM ဖိုင် မရှိ");
    data.insert("wiki.no_zim_hint", "ဝီကီပီးဒီးယား သုံးရန် ZIM ဖိုင် ထည့်ပါ");
    data.insert("wiki.no_zim_loaded", "ZIM ဖိုင် မတင်ရသေး");
    data.insert("wiki.loaded_zims", "တင်ထားသော ZIM များ");
    data.insert("wiki.add_btn", "ထည့်");
    data.insert("wiki.add_hint", "ZIM ဖိုင် လမ်းကြောင်း အပြည့်အစုံ ထည့်ပါ");
    data.insert("wiki.name", "အမည်");
    data.insert("wiki.path", "လမ်းကြောင်း");
    data.insert("wiki.action", "လုပ်ဆောင်ချက်");
    data.insert("wiki.zim_added", " ZIM ထည့်ပြီး: ");
    data.insert("wiki.no_new_zim", "ZIM အသစ် မရှိ");
    data.insert("wiki.refresh_failed", "ပြန်လည်စတင် မအောင်မြင်");
    data.insert("wiki.enter_path", "လမ်းကြောင်း ထည့်ပါ");
    data.insert("wiki.add_failed", "ထည့်သွင်း မအောင်မြင်");
    data.insert("wiki.remove_confirm", "ဤ ZIM ကို ဖယ်ရှားမလား?");
    data.insert("wiki.zim_removed", "ZIM ဖယ်ရှားပြီး: ");
    data.insert("wiki.remove_failed", "ဖယ်ရှား မအောင်မြင်");
    data.insert("wiki.open_folder_msg", "ဖိုဒါကို ဖိုင်မန်နေဂျာတွင် ဖွင့်ပါ:");

    // SRS
    data.insert("srs.title", "SRS ပြန်လည်သုံးသပ်");
    data.insert("srs.cards", "ကတ် စာရင်း");
    data.insert("srs.streak", "ဆက်တိုက်");
    data.insert("srs.show_answer", "အဖြေ ပြ");
    data.insert("srs.again", "ထပ်မံ");
    data.insert("srs.hard", "ခက်");
    data.insert("srs.good", "ကောင်း");
    data.insert("srs.easy", "လွယ်");
    data.insert("srs.complete", "🎉 ယနေ့ ပြန်လည်သုံးသပ်မှု ပြီးဆုံး!");
    data.insert("srs.no_cards", "သုံးသပ်ရန် ကတ် မရှိ");
    data.insert("srs.progress", "ကတ်");
    data.insert("srs.today_review", "ယနေ့ သုံးသပ်ရန်");
    data.insert("srs.new_cards", "အသစ်");
    data.insert("srs.learning", "လေ့လာနေ");
    data.insert("srs.mature", "ကျွမ်းကျင်");
    data.insert("srs.create_from_notes", "မှတ်စုများမှ ကတ် ဖန်တီး");
    data.insert("srs.create_cards", "ကတ် ဖန်တီး");
    data.insert("srs.cards_created", "ကတ် ဖန်တီးပြီး");
    data.insert("srs.no_cards_extracted", "ထုတ်ယူရန် ကတ် မရှိ");
    data.insert("srs.repetitions", "ထပ်ခါထပ်ခါ");
    data.insert("srs.streak_days", "ဆက်တိုက်");
    data.insert("srs.start_review", "သုံးသပ်မှု စတင်");
    data.insert("srs.no_cards_yet", "ကတ် မရှိသေး");
    data.insert("srs.create_hint", "မှတ်စု ဖွင့်ပြီး \"ကတ် ဖန်တီး\" နှိပ်ပါ");
    data.insert("srs.interval", "ကြားကာလ");
    data.insert("srs.days", " ရက်");
    data.insert("srs.delete_card_confirm", "ဤကတ်ကို ဖျက်မလား?");
    data.insert("srs.delete_failed", "ဖျက်မှု မအောင်မြင်");

    // Security
    data.insert("security.title", "လုံခြုံရေး ဆက်တင်");
    data.insert("security.pin_not_set", "PIN မသတ်မှတ်ရသေး");
    data.insert("security.pin_enabled", "PIN ဖွင့်ထား");
    data.insert("security.locked", "လော့ခ်ထား - PIN ထည့်ပါ");
    data.insert("security.pin_input", "PIN ထည့်ပါ (၆-၃၂ စာလုံး)");
    data.insert("security.set_pin", "PIN သတ်မှတ်");
    data.insert("security.remove_pin", "PIN ဖယ်ရှား");
    data.insert("security.lock", "လော့ခ်");
    data.insert("security.unlock", "လော့ခ်ဖွင့်");
    data.insert("security.wrong_pin", "PIN မှား");
    data.insert("security.pin_set_success", "PIN သတ်မှတ်ပြီး");
    data.insert("security.pin_removed", "PIN ဖယ်ရှားပြီး");
    data.insert("security.unlocked", "လော့ခ်ဖွင့်ပြီး");
    data.insert("security.pin_min_length", "PIN အနည်းဆုံး ၆ စာလုံး လိုအပ်");
    data.insert("security.enter_current_pin", "လက်ရှိ PIN ထည့်ပါ");

    // Backup
    data.insert("backup.title", "အရန်သိမ်း");
    data.insert("backup.info", "အရန်သိမ်း အချက်အလက်");
    data.insert("backup.now", "ယခု အရန်သိမ်း");
    data.insert("backup.complete", "အရန်သိမ်း ပြီးဆုံး");
    data.insert("backup.no_changes", "ပြောင်းလဲမှု မရှိ (အရန်သိမ်း ကျော်)");

    // Split view
    data.insert("split.select_note", "ဘယ်ဘက်မှ မှတ်စု ရွေးချယ်ပါ");

    // Common buttons
    data.insert("common.confirm", "အတည်ပြု");
    data.insert("common.cancel", "ပယ်ဖျက်");
    data.insert("common.close", "ပိတ်");
    data.insert("common.loading", "ဖွင့်နေ...");
    data.insert("common.error", "အမှား ဖြစ်ပွား");
    data.insert("common.success", "အောင်မြင်");

    // Settings
    data.insert("settings.title", "ဆက်တင်");
    data.insert("settings.language", "ဘာသာစကား");
    data.insert("settings.language_desc", "သင့် ဘာသာစကား ရွေးချယ်ပါ");
    data.insert("settings.stats", "စာရင်းအင်း");
    data.insert("settings.version", "ဗားရှင်း");
    data.insert("settings.storage", "သိုလှောင်မှု");
    data.insert("settings.about", "အကြောင်း");

    // === USB ===
    data.insert("usb.title", "USB စင့်ခ်");
    data.insert("usb.scan", "စကန်");
    data.insert("usb.scanning", "USB ရှာဖွေနေ");
    data.insert("usb.no_usb", "Lazarus USB မတွေ့ပါ");
    data.insert(
        "usb.no_usb_hint",
        "lazarus.sync ပါသော USB ထည့်ပါ သို့မဟုတ် အောက်တွင် အစပြု",
    );
    data.insert("usb.error", "စကန် မအောင်မြင်");
    data.insert("usb.init_title", "USB အစပြု");
    data.insert("usb.init_desc", "အော့ဖ်လိုင်း မျှဝေရန် Lazarus USB အသစ် ဖန်တီး");
    data.insert("usb.init_btn", "အစပြု");
    data.insert("usb.init_error", "အစပြု မအောင်မြင်");
    data.insert("usb.enter_path", "USB လမ်းကြောင်း ထည့်ပါ");
    data.insert("usb.notes", "မှတ်စု");
    data.insert("usb.posts", "ပို့စ်");
    data.insert("usb.packages", "Package");
    data.insert("usb.sync", "စင့်ခ်");
    data.insert("usb.export", "ထုတ်ယူ");
    data.insert("usb.import", "သွင်းယူ");

    Translations::new(data)
}
