//! 繁體中文 - Traditional Chinese translations

use super::Translations;
use std::collections::HashMap;

pub fn translations() -> Translations {
    let mut data = HashMap::new();

    // Common
    data.insert("app.name", "Lazarus");
    data.insert("app.tagline", "離線個人知識管理");
    data.insert("nav.notes", "筆記");
    data.insert("nav.search", "搜尋");
    data.insert("nav.wiki", "維基");

    // Home
    data.insert("home.notes", "筆記");
    data.insert("home.streak", "連續學習");
    data.insert("home.day", "天");
    data.insert("home.quick_start", "快速開始");
    data.insert("home.new_note", "新增筆記");
    data.insert("home.note_list", "筆記列表");
    data.insert("home.split_view", "分割檢視");
    data.insert("home.srs_review", "SRS複習");
    data.insert("home.search", "搜尋");
    data.insert("home.zim_manage", "ZIM管理");
    data.insert("home.security", "安全性");
    data.insert("home.shortcuts", "鍵盤快捷鍵");
    data.insert("home.shortcut.new_note", "新增筆記");
    data.insert("home.shortcut.search", "搜尋");
    data.insert("home.shortcut.help", "說明");

    // Editor
    data.insert("editor.title_placeholder", "輸入標題");
    data.insert("editor.content_placeholder", "在這裡開始寫作...");
    data.insert("editor.tags", "標籤");
    data.insert("editor.tags_placeholder", "標籤1, 標籤2, ...");
    data.insert("editor.edit_note", "編輯筆記");
    data.insert("editor.encrypt", "切換加密");
    data.insert("editor.focus", "專注");
    data.insert("editor.fullscreen", "全螢幕");
    data.insert("editor.save", "儲存");
    data.insert("editor.saved", "已儲存");
    data.insert("editor.auto_saved", "自動儲存");
    data.insert("editor.changed", "已變更...");
    data.insert("editor.words", "字");
    data.insert("editor.save_complete", "💾 儲存完成!");
    data.insert("editor.encrypt_on", "🔒 加密已啟用");
    data.insert("editor.encrypt_off", "🔓 加密已停用");
    data.insert("editor.pin_required", "請先設定PIN（安全性選單）");
    data.insert("editor.pin_locked", "請先解鎖PIN");

    // Notes list
    data.insert("notes.title", "筆記");
    data.insert("notes.export", "匯出");
    data.insert("notes.import", "匯入");
    data.insert("notes.no_notes", "還沒有筆記");
    data.insert("notes.create_first", "建立您的第一個筆記！");
    data.insert("notes.no_title", "無標題");
    data.insert("notes.edit", "編輯");
    data.insert("notes.delete", "刪除");
    data.insert("notes.delete_confirm", "確定要刪除嗎？");
    data.insert("notes.encrypted", "已加密");
    data.insert("notes.created", "建立於");
    data.insert("notes.updated", "更新於");
    data.insert("notes.find_duplicates", "尋找重複");
    data.insert("notes.select_all", "全選");
    data.insert("notes.selected", "已選取");
    data.insert("notes.create_package", "建立套件");
    data.insert("notes.duplicates_title", "重複筆記");
    data.insert("notes.no_duplicates", "沒有重複的筆記");
    data.insert("notes.export_package", "匯出套件");
    data.insert("notes.import_package", "匯入套件");
    data.insert("notes.package_title", "套件標題");
    data.insert("notes.package_title_placeholder", "例如：物理101");
    data.insert("notes.package_author", "作者");
    data.insert("notes.package_author_placeholder", "您的名字");
    data.insert("notes.package_description", "描述");
    data.insert("notes.package_description_placeholder", "套件描述（選填）");
    data.insert("notes.drop_file", "將.laz檔案拖放到這裡");
    data.insert("notes.or", "或");
    data.insert("notes.select_file", "選擇檔案");

    // Search
    data.insert("search.title", "搜尋");
    data.insert("search.placeholder", "輸入搜尋詞...");
    data.insert("search.button", "搜尋");
    data.insert("search.results", "搜尋結果");
    data.insert("search.no_results", "找不到結果");
    data.insert("search.try_different", "嘗試不同的搜尋詞");
    data.insert("search.tips", "搜尋提示");
    data.insert("search.tip1", "多個詞搜尋包含所有詞的結果");
    data.insert("search.tip2", "同時搜尋筆記和維基百科");
    data.insert("search.tip3", "標題符合優先顯示");

    // Wiki
    data.insert("wiki.search", "維基搜尋");
    data.insert("wiki.recent_articles", "Recent Articles");
    data.insert("wiki.manage", "ZIM管理");
    data.insert("wiki.status", "狀態");
    data.insert("wiki.loaded", "已載入");
    data.insert("wiki.directory", "ZIM目錄");
    data.insert("wiki.directory_hint", "將.zim檔案複製到此資料夾並重新整理");
    data.insert("wiki.refresh", "重新整理");
    data.insert("wiki.open_folder", "開啟資料夾");
    data.insert("wiki.add", "新增ZIM");
    data.insert("wiki.add_placeholder", "輸入ZIM檔案路徑...");
    data.insert("wiki.loaded_files", "已載入的ZIM檔案");
    data.insert("wiki.remove", "移除");
    data.insert("wiki.no_zim", "沒有ZIM檔案");
    data.insert("wiki.no_zim_hint", "新增ZIM檔案以使用維基百科");
    data.insert("wiki.no_zim_loaded", "沒有載入ZIM檔案");
    data.insert("wiki.loaded_zims", "已載入的ZIM");
    data.insert("wiki.add_btn", "新增");
    data.insert("wiki.add_hint", "輸入ZIM檔案的完整路徑。");
    data.insert("wiki.name", "名稱");
    data.insert("wiki.path", "路徑");
    data.insert("wiki.action", "操作");
    data.insert("wiki.zim_added", " ZIM已新增: ");
    data.insert("wiki.no_new_zim", "沒有新的ZIM檔案");
    data.insert("wiki.refresh_failed", "重新整理失敗");
    data.insert("wiki.enter_path", "請輸入路徑");
    data.insert("wiki.add_failed", "新增失敗");
    data.insert("wiki.remove_confirm", "移除這個ZIM？");
    data.insert("wiki.zim_removed", "ZIM已移除: ");
    data.insert("wiki.remove_failed", "移除失敗");
    data.insert("wiki.open_folder_msg", "請在檔案總管中開啟資料夾:");

    // SRS
    data.insert("srs.title", "SRS複習");
    data.insert("srs.cards", "卡片列表");
    data.insert("srs.streak", "連續");
    data.insert("srs.show_answer", "顯示答案");
    data.insert("srs.again", "重來");
    data.insert("srs.hard", "困難");
    data.insert("srs.good", "良好");
    data.insert("srs.easy", "簡單");
    data.insert("srs.complete", "🎉 今日複習完成!");
    data.insert("srs.no_cards", "沒有需要複習的卡片");
    data.insert("srs.progress", "卡片");
    data.insert("srs.today_review", "今日複習");
    data.insert("srs.new_cards", "新卡片");
    data.insert("srs.learning", "學習中");
    data.insert("srs.mature", "已掌握");
    data.insert("srs.create_from_notes", "從筆記建立卡片");
    data.insert("srs.create_cards", "建立卡片");
    data.insert("srs.cards_created", "張卡片已建立");
    data.insert("srs.no_cards_extracted", "沒有可擷取的卡片");
    data.insert("srs.repetitions", "重複次數");
    data.insert("srs.streak_days", "連續天數");
    data.insert("srs.start_review", "開始複習");
    data.insert("srs.no_cards_yet", "還沒有卡片。");
    data.insert("srs.create_hint", "開啟筆記並點擊「建立卡片」按鈕。");
    data.insert("srs.interval", "間隔");
    data.insert("srs.days", "天");
    data.insert("srs.delete_card_confirm", "刪除這張卡片？");
    data.insert("srs.delete_failed", "刪除失敗");

    // Security
    data.insert("security.title", "安全性設定");
    data.insert("security.pin_not_set", "未設定PIN");
    data.insert("security.pin_enabled", "PIN已啟用");
    data.insert("security.locked", "已鎖定 - 輸入PIN");
    data.insert("security.pin_input", "輸入PIN（6-32個字元）");
    data.insert("security.set_pin", "設定PIN");
    data.insert("security.remove_pin", "移除PIN");
    data.insert("security.lock", "鎖定");
    data.insert("security.unlock", "解鎖");
    data.insert("security.wrong_pin", "PIN錯誤");
    data.insert("security.pin_set_success", "PIN已設定");
    data.insert("security.pin_removed", "PIN已移除");
    data.insert("security.unlocked", "已解鎖");
    data.insert("security.pin_min_length", "PIN至少需要6個字元");
    data.insert("security.enter_current_pin", "輸入目前的PIN");

    // Backup
    data.insert("backup.title", "備份");
    data.insert("backup.info", "備份資訊");
    data.insert("backup.now", "立即備份");
    data.insert("backup.complete", "備份完成");
    data.insert("backup.no_changes", "無變更（略過備份）");

    // Split view
    data.insert("split.select_note", "從左側選擇筆記");

    // Common buttons
    data.insert("common.confirm", "確認");
    data.insert("common.cancel", "取消");
    data.insert("common.close", "關閉");
    data.insert("common.loading", "載入中...");
    data.insert("common.error", "發生錯誤");
    data.insert("common.success", "成功");

    // Settings
    data.insert("settings.title", "設定");
    data.insert("settings.language", "語言");
    data.insert("settings.language_desc", "選擇您偏好的語言");
    data.insert("settings.stats", "統計");
    data.insert("settings.version", "版本");
    data.insert("settings.storage", "儲存空間");
    data.insert("settings.about", "關於");

    // Knowledge Graph & Footer
    data.insert("home.knowledge_graph", "知識圖譜");
    data.insert("app.footer", "知識屬於每個人");

    // Knowledge Graph & Footer
    data.insert("home.knowledge_graph", "知識圖譜");
    data.insert("app.footer", "知識屬於每個人");

    // === USB ===
    data.insert("usb.title", "USB同步");
    data.insert("usb.scan", "掃描");
    data.insert("usb.scanning", "正在搜尋USB裝置");
    data.insert("usb.no_usb", "未偵測到Lazarus USB");
    data.insert("usb.no_usb_hint", "插入帶有lazarus.sync的USB或在下方初始化");
    data.insert("usb.error", "掃描失敗");
    data.insert("usb.init_title", "初始化USB");
    data.insert("usb.init_desc", "建立新的Lazarus USB用於離線共享");
    data.insert("usb.init_btn", "初始化");
    data.insert("usb.init_error", "初始化失敗");
    data.insert("usb.enter_path", "輸入USB路徑");
    data.insert("usb.notes", "筆記");
    data.insert("usb.posts", "貼文");
    data.insert("usb.packages", "套件");
    data.insert("usb.sync", "同步");
    data.insert("usb.export", "匯出");
    data.insert("usb.import", "匯入");
    data.insert("home.usb_sync", "USB同步");
    data.insert("home.shortcut.wiki", "維基");
    data.insert("home.shortcut.usb", "USB");
    data.insert("home.shortcut.graph", "圖表");
    data.insert("home.shortcut.review", "複習");
    Translations::new(data)
}
