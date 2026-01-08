//! 廣東話 - Cantonese translations

use super::Translations;
use std::collections::HashMap;

pub fn translations() -> Translations {
    let mut data = HashMap::new();

    // Common
    data.insert("app.name", "Lazarus");
    data.insert("app.tagline", "離線個人知識管理");
    data.insert("nav.notes", "筆記");
    data.insert("nav.search", "搵嘢");
    data.insert("nav.wiki", "維基");

    // Home
    data.insert("home.notes", "筆記");
    data.insert("home.streak", "連續學習");
    data.insert("home.day", "日");
    data.insert("home.quick_start", "快速開始");
    data.insert("home.new_note", "新筆記");
    data.insert("home.note_list", "筆記列表");
    data.insert("home.split_view", "分割畫面");
    data.insert("home.srs_review", "SRS溫習");
    data.insert("home.search", "搵嘢");
    data.insert("home.zim_manage", "ZIM管理");
    data.insert("home.security", "保安");
    data.insert("home.shortcuts", "鍵盤快捷鍵");
    data.insert("home.shortcut.new_note", "新筆記");
    data.insert("home.shortcut.search", "搵嘢");
    data.insert("home.shortcut.help", "幫助");

    // Editor
    data.insert("editor.title_placeholder", "輸入標題");
    data.insert("editor.content_placeholder", "喺度開始寫...");
    data.insert("editor.tags", "標籤");
    data.insert("editor.tags_placeholder", "標籤1, 標籤2, ...");
    data.insert("editor.edit_note", "編輯筆記");
    data.insert("editor.encrypt", "切換加密");
    data.insert("editor.focus", "專注");
    data.insert("editor.fullscreen", "全螢幕");
    data.insert("editor.save", "儲存");
    data.insert("editor.saved", "已儲存");
    data.insert("editor.auto_saved", "自動儲存咗");
    data.insert("editor.changed", "改咗...");
    data.insert("editor.words", "字");
    data.insert("editor.save_complete", "💾 儲存完成!");
    data.insert("editor.encrypt_on", "🔒 加密開咗");
    data.insert("editor.encrypt_off", "🔓 加密閂咗");
    data.insert("editor.pin_required", "請先設定PIN（保安選單）");
    data.insert("editor.pin_locked", "請先解鎖PIN");

    // Notes list
    data.insert("notes.title", "筆記");
    data.insert("notes.export", "匯出");
    data.insert("notes.import", "匯入");
    data.insert("notes.no_notes", "仲未有筆記");
    data.insert("notes.create_first", "寫你第一個筆記啦！");
    data.insert("notes.no_title", "無標題");
    data.insert("notes.edit", "編輯");
    data.insert("notes.delete", "刪除");
    data.insert("notes.delete_confirm", "確定要刪除？");
    data.insert("notes.encrypted", "已加密");
    data.insert("notes.created", "建立於");
    data.insert("notes.updated", "更新於");
    data.insert("notes.find_duplicates", "搵重複");
    data.insert("notes.select_all", "揀晒");
    data.insert("notes.selected", "已揀");
    data.insert("notes.create_package", "建立套件");
    data.insert("notes.duplicates_title", "重複筆記");
    data.insert("notes.no_duplicates", "無重複嘅筆記");
    data.insert("notes.export_package", "匯出套件");
    data.insert("notes.import_package", "匯入套件");
    data.insert("notes.package_title", "套件標題");
    data.insert("notes.package_title_placeholder", "例如：物理101");
    data.insert("notes.package_author", "作者");
    data.insert("notes.package_author_placeholder", "你個名");
    data.insert("notes.package_description", "描述");
    data.insert("notes.package_description_placeholder", "套件描述（可選）");
    data.insert("notes.drop_file", "將.laz檔案拖入呢度");
    data.insert("notes.or", "或者");
    data.insert("notes.select_file", "揀檔案");

    // Search
    data.insert("search.title", "搵嘢");
    data.insert("search.placeholder", "輸入搜尋詞...");
    data.insert("search.button", "搵");
    data.insert("search.results", "搜尋結果");
    data.insert("search.no_results", "搵唔到結果");
    data.insert("search.try_different", "試吓其他搜尋詞");
    data.insert("search.tips", "搜尋貼士");
    data.insert("search.tip1", "多個詞搵包含所有詞嘅結果");
    data.insert("search.tip2", "同時搵筆記同維基百科");
    data.insert("search.tip3", "標題符合優先顯示");

    // Wiki
    data.insert("wiki.search", "維基搵嘢");
    data.insert("wiki.recent_articles", "Recent Articles");
    data.insert("wiki.manage", "ZIM管理");
    data.insert("wiki.status", "狀態");
    data.insert("wiki.loaded", "已載入");
    data.insert("wiki.directory", "ZIM目錄");
    data.insert(
        "wiki.directory_hint",
        "將.zim檔案複製到呢個資料夾然後重新整理",
    );
    data.insert("wiki.refresh", "重新整理");
    data.insert("wiki.open_folder", "開資料夾");
    data.insert("wiki.add", "加ZIM");
    data.insert("wiki.add_placeholder", "輸入ZIM檔案路徑...");
    data.insert("wiki.loaded_files", "已載入嘅ZIM檔案");
    data.insert("wiki.remove", "移除");
    data.insert("wiki.no_zim", "無ZIM檔案");
    data.insert("wiki.no_zim_hint", "加ZIM檔案嚟用維基百科");
    data.insert("wiki.no_zim_loaded", "無載入ZIM檔案");
    data.insert("wiki.loaded_zims", "已載入嘅ZIM");
    data.insert("wiki.add_btn", "加");
    data.insert("wiki.add_hint", "輸入ZIM檔案嘅完整路徑。");
    data.insert("wiki.name", "名");
    data.insert("wiki.path", "路徑");
    data.insert("wiki.action", "操作");
    data.insert("wiki.zim_added", " ZIM加咗: ");
    data.insert("wiki.no_new_zim", "無新ZIM檔案");
    data.insert("wiki.refresh_failed", "重新整理失敗");
    data.insert("wiki.enter_path", "請輸入路徑");
    data.insert("wiki.add_failed", "加失敗");
    data.insert("wiki.remove_confirm", "移除呢個ZIM？");
    data.insert("wiki.zim_removed", "ZIM移除咗: ");
    data.insert("wiki.remove_failed", "移除失敗");
    data.insert("wiki.open_folder_msg", "請喺檔案總管開資料夾:");

    // SRS
    data.insert("srs.title", "SRS溫習");
    data.insert("srs.cards", "卡片列表");
    data.insert("srs.streak", "連續");
    data.insert("srs.show_answer", "睇答案");
    data.insert("srs.again", "再嚟");
    data.insert("srs.hard", "難");
    data.insert("srs.good", "OK");
    data.insert("srs.easy", "易");
    data.insert("srs.complete", "🎉 今日溫習完成!");
    data.insert("srs.no_cards", "無需要溫習嘅卡片");
    data.insert("srs.progress", "卡片");
    data.insert("srs.today_review", "今日溫習");
    data.insert("srs.new_cards", "新卡片");
    data.insert("srs.learning", "學緊");
    data.insert("srs.mature", "識咗");
    data.insert("srs.create_from_notes", "由筆記建立卡片");
    data.insert("srs.create_cards", "建立卡片");
    data.insert("srs.cards_created", "張卡片建立咗");
    data.insert("srs.no_cards_extracted", "無可以擷取嘅卡片");
    data.insert("srs.repetitions", "重複次數");
    data.insert("srs.streak_days", "連續日數");
    data.insert("srs.start_review", "開始溫習");
    data.insert("srs.no_cards_yet", "仲未有卡片。");
    data.insert("srs.create_hint", "開筆記然後撳「建立卡片」掣。");
    data.insert("srs.interval", "間隔");
    data.insert("srs.days", "日");
    data.insert("srs.delete_card_confirm", "刪除呢張卡片？");
    data.insert("srs.delete_failed", "刪除失敗");

    // Security
    data.insert("security.title", "保安設定");
    data.insert("security.pin_not_set", "未設定PIN");
    data.insert("security.pin_enabled", "PIN開咗");
    data.insert("security.locked", "鎖咗 - 輸入PIN");
    data.insert("security.pin_input", "輸入PIN（6-32個字）");
    data.insert("security.set_pin", "設定PIN");
    data.insert("security.remove_pin", "移除PIN");
    data.insert("security.lock", "鎖");
    data.insert("security.unlock", "解鎖");
    data.insert("security.wrong_pin", "PIN錯咗");
    data.insert("security.pin_set_success", "PIN設定咗");
    data.insert("security.pin_removed", "PIN移除咗");
    data.insert("security.unlocked", "解鎖咗");
    data.insert("security.pin_min_length", "PIN至少要6個字");
    data.insert("security.enter_current_pin", "輸入而家嘅PIN");

    // Backup
    data.insert("backup.title", "備份");
    data.insert("backup.info", "備份資訊");
    data.insert("backup.now", "即刻備份");
    data.insert("backup.complete", "備份完成");
    data.insert("backup.no_changes", "無改動（跳過備份）");

    // Split view
    data.insert("split.select_note", "由左邊揀筆記");

    // Common buttons
    data.insert("common.confirm", "確認");
    data.insert("common.cancel", "取消");
    data.insert("common.close", "閂");
    data.insert("common.loading", "載入緊...");
    data.insert("common.error", "出錯");
    data.insert("common.success", "成功");

    // Settings
    data.insert("settings.title", "設定");
    data.insert("settings.language", "語言");
    data.insert("settings.language_desc", "揀你鍾意嘅語言");
    data.insert("settings.stats", "統計");
    data.insert("settings.version", "版本");
    data.insert("settings.storage", "儲存空間");
    data.insert("settings.about", "關於");

    // Knowledge Graph & Footer
    data.insert("home.knowledge_graph", "知識圖譜");
    data.insert("app.footer", "知識屬於每個人");

    // === USB ===
    data.insert("usb.title", "USB同步");
    data.insert("usb.scan", "掃描");
    data.insert("usb.scanning", "搵緊USB裝置");
    data.insert("usb.no_usb", "搵唔到Lazarus USB");
    data.insert("usb.no_usb_hint", "插入有lazarus.sync嘅USB或者喺下面初始化");
    data.insert("usb.error", "掃描失敗");
    data.insert("usb.init_title", "初始化USB");
    data.insert("usb.init_desc", "建立新嘅Lazarus USB用嚟離線分享");
    data.insert("usb.init_btn", "初始化");
    data.insert("usb.init_error", "初始化失敗");
    data.insert("usb.enter_path", "輸入USB路徑");
    data.insert("usb.notes", "筆記");
    data.insert("usb.posts", "帖子");
    data.insert("usb.packages", "套件");
    data.insert("usb.sync", "同步");
    data.insert("usb.export", "匯出");
    data.insert("usb.import", "匯入");
    data.insert("home.usb_sync", "USB同步");
    data.insert("home.shortcut.wiki", "維基");
    data.insert("home.shortcut.usb", "USB");
    data.insert("home.shortcut.graph", "圖表");
    data.insert("home.shortcut.review", "複習");

    // === Posts ===
    data.insert("posts.title", "帖子");
    data.insert("posts.new_post", "新帖");
    data.insert("posts.no_posts", "未有帖子");
    data.insert("posts.be_first", "做第一個發帖嘅人！");
    data.insert("posts.author", "你個名");
    data.insert("posts.title_placeholder", "標題");
    data.insert("posts.content_placeholder", "內容...");
    data.insert("posts.tags_placeholder", "標籤（用逗號分開）");
    data.insert("posts.post_btn", "發帖");
    data.insert("posts.replies", "回覆");
    data.insert("posts.delete_confirm", "刪除呢個帖？");
    data.insert("posts.write_reply", "寫回覆...");
    data.insert("posts.reply_btn", "回覆");
    // === Q&A ===
    data.insert("qna.title", "問答");
    data.insert("qna.ask_question", "問問題");
    data.insert("qna.no_questions", "未有問題");
    data.insert("qna.be_first", "做第一個問嘅人！");
    data.insert("qna.question_title", "問題標題");
    data.insert("qna.question_content", "描述你嘅問題...");
    data.insert("qna.post_question", "發問題");
    data.insert("qna.answers", "答案");
    data.insert("qna.write_answer", "寫你嘅答案...");
    data.insert("qna.post_answer", "發答案");
    data.insert("qna.accept", "採納");
    data.insert("qna.accepted", "已採納");
    data.insert("qna.delete_confirm", "刪除呢條問題？");

    Translations::new(data)
}
