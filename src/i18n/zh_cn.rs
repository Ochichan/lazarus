//! 简体中文 - Simplified Chinese translations

use super::Translations;
use std::collections::HashMap;

pub fn translations() -> Translations {
    let mut data = HashMap::new();

    // Common
    data.insert("app.name", "Lazarus");
    data.insert("app.tagline", "离线个人知识管理");
    data.insert("nav.notes", "笔记");
    data.insert("nav.search", "搜索");
    data.insert("nav.wiki", "维基");

    // Home
    data.insert("home.notes", "笔记");
    data.insert("home.streak", "连续学习");
    data.insert("home.day", "天");
    data.insert("home.quick_start", "快速开始");
    data.insert("home.new_note", "新建笔记");
    data.insert("home.note_list", "笔记列表");
    data.insert("home.split_view", "分屏视图");
    data.insert("home.srs_review", "SRS复习");
    data.insert("home.search", "搜索");
    data.insert("home.zim_manage", "ZIM管理");
    data.insert("home.security", "安全");
    data.insert("home.shortcuts", "键盘快捷键");
    data.insert("home.shortcut.new_note", "新建笔记");
    data.insert("home.shortcut.search", "搜索");
    data.insert("home.shortcut.help", "帮助");

    // Editor
    data.insert("editor.title_placeholder", "输入标题");
    data.insert("editor.content_placeholder", "在这里开始写作...");
    data.insert("editor.tags", "标签");
    data.insert("editor.tags_placeholder", "标签1, 标签2, ...");
    data.insert("editor.edit_note", "编辑笔记");
    data.insert("editor.encrypt", "切换加密");
    data.insert("editor.focus", "专注");
    data.insert("editor.fullscreen", "全屏");
    data.insert("editor.save", "保存");
    data.insert("editor.saved", "已保存");
    data.insert("editor.auto_saved", "自动保存");
    data.insert("editor.changed", "已更改...");
    data.insert("editor.words", "字");
    data.insert("editor.save_complete", "💾 保存完成!");
    data.insert("editor.encrypt_on", "🔒 加密已启用");
    data.insert("editor.encrypt_off", "🔓 加密已禁用");
    data.insert("editor.pin_required", "请先设置PIN（安全菜单）");
    data.insert("editor.pin_locked", "请先解锁PIN");

    // Notes list
    data.insert("notes.title", "笔记");
    data.insert("notes.export", "导出");
    data.insert("notes.import", "导入");
    data.insert("notes.no_notes", "还没有笔记");
    data.insert("notes.create_first", "创建你的第一个笔记！");
    data.insert("notes.no_title", "无标题");
    data.insert("notes.edit", "编辑");
    data.insert("notes.delete", "删除");
    data.insert("notes.delete_confirm", "确定要删除吗？");
    data.insert("notes.encrypted", "已加密");
    data.insert("notes.created", "创建于");
    data.insert("notes.updated", "更新于");
    data.insert("notes.find_duplicates", "查找重复");
    data.insert("notes.select_all", "全选");
    data.insert("notes.selected", "已选择");
    data.insert("notes.create_package", "创建包");
    data.insert("notes.duplicates_title", "重复笔记");
    data.insert("notes.no_duplicates", "没有重复笔记");
    data.insert("notes.export_package", "导出包");
    data.insert("notes.import_package", "导入包");
    data.insert("notes.package_title", "包标题");
    data.insert("notes.package_title_placeholder", "例如：物理101");
    data.insert("notes.package_author", "作者");
    data.insert("notes.package_author_placeholder", "你的名字");
    data.insert("notes.package_description", "描述");
    data.insert("notes.package_description_placeholder", "包描述（可选）");
    data.insert("notes.drop_file", "将.laz文件拖放到这里");
    data.insert("notes.or", "或");
    data.insert("notes.select_file", "选择文件");

    // Search
    data.insert("search.title", "搜索");
    data.insert("search.placeholder", "输入搜索词...");
    data.insert("search.button", "搜索");
    data.insert("search.results", "搜索结果");
    data.insert("search.no_results", "未找到结果");
    data.insert("search.try_different", "尝试不同的搜索词");
    data.insert("search.tips", "搜索提示");
    data.insert("search.tip1", "多个词搜索包含所有词的结果");
    data.insert("search.tip2", "同时搜索笔记和维基百科");
    data.insert("search.tip3", "标题匹配优先显示");

    // Wiki
    data.insert("wiki.search", "维基搜索");
    data.insert("wiki.recent_articles", "Recent Articles");
    data.insert("wiki.manage", "ZIM管理");
    data.insert("wiki.status", "状态");
    data.insert("wiki.loaded", "已加载");
    data.insert("wiki.directory", "ZIM目录");
    data.insert("wiki.directory_hint", "将.zim文件复制到此文件夹并刷新");
    data.insert("wiki.refresh", "刷新");
    data.insert("wiki.open_folder", "打开文件夹");
    data.insert("wiki.add", "添加ZIM");
    data.insert("wiki.add_placeholder", "输入ZIM文件路径...");
    data.insert("wiki.loaded_files", "已加载的ZIM文件");
    data.insert("wiki.remove", "移除");
    data.insert("wiki.no_zim", "没有ZIM文件");
    data.insert("wiki.no_zim_hint", "添加ZIM文件以使用维基百科");
    data.insert("wiki.no_zim_loaded", "没有加载ZIM文件");
    data.insert("wiki.loaded_zims", "已加载的ZIM");
    data.insert("wiki.add_btn", "添加");
    data.insert("wiki.add_hint", "输入ZIM文件的完整路径。");
    data.insert("wiki.name", "名称");
    data.insert("wiki.path", "路径");
    data.insert("wiki.action", "操作");
    data.insert("wiki.zim_added", " ZIM已添加: ");
    data.insert("wiki.no_new_zim", "没有新的ZIM文件");
    data.insert("wiki.refresh_failed", "刷新失败");
    data.insert("wiki.enter_path", "请输入路径");
    data.insert("wiki.add_failed", "添加失败");
    data.insert("wiki.remove_confirm", "移除这个ZIM？");
    data.insert("wiki.zim_removed", "ZIM已移除: ");
    data.insert("wiki.remove_failed", "移除失败");
    data.insert("wiki.open_folder_msg", "请在文件管理器中打开文件夹:");

    // SRS
    data.insert("srs.title", "SRS复习");
    data.insert("srs.cards", "卡片列表");
    data.insert("srs.streak", "连续");
    data.insert("srs.show_answer", "显示答案");
    data.insert("srs.again", "重来");
    data.insert("srs.hard", "困难");
    data.insert("srs.good", "良好");
    data.insert("srs.easy", "简单");
    data.insert("srs.complete", "🎉 今日复习完成!");
    data.insert("srs.no_cards", "没有需要复习的卡片");
    data.insert("srs.progress", "卡片");
    data.insert("srs.today_review", "今日复习");
    data.insert("srs.new_cards", "新卡片");
    data.insert("srs.learning", "学习中");
    data.insert("srs.mature", "已掌握");
    data.insert("srs.create_from_notes", "从笔记创建卡片");
    data.insert("srs.create_cards", "创建卡片");
    data.insert("srs.cards_created", "张卡片已创建");
    data.insert("srs.no_cards_extracted", "没有可提取的卡片");
    data.insert("srs.repetitions", "重复次数");
    data.insert("srs.streak_days", "连续天数");
    data.insert("srs.start_review", "开始复习");
    data.insert("srs.no_cards_yet", "还没有卡片。");
    data.insert("srs.create_hint", "打开笔记并点击\"创建卡片\"按钮。");
    data.insert("srs.interval", "间隔");
    data.insert("srs.days", "天");
    data.insert("srs.delete_card_confirm", "删除这张卡片？");
    data.insert("srs.delete_failed", "删除失败");

    // Security
    data.insert("security.title", "安全设置");
    data.insert("security.pin_not_set", "未设置PIN");
    data.insert("security.pin_enabled", "PIN已启用");
    data.insert("security.locked", "已锁定 - 输入PIN");
    data.insert("security.pin_input", "输入PIN（6-32个字符）");
    data.insert("security.set_pin", "设置PIN");
    data.insert("security.remove_pin", "移除PIN");
    data.insert("security.lock", "锁定");
    data.insert("security.unlock", "解锁");
    data.insert("security.wrong_pin", "PIN错误");
    data.insert("security.pin_set_success", "PIN已设置");
    data.insert("security.pin_removed", "PIN已移除");
    data.insert("security.unlocked", "已解锁");
    data.insert("security.pin_min_length", "PIN至少需要6个字符");
    data.insert("security.enter_current_pin", "输入当前PIN");

    // Backup
    data.insert("backup.title", "备份");
    data.insert("backup.info", "备份信息");
    data.insert("backup.now", "立即备份");
    data.insert("backup.complete", "备份完成");
    data.insert("backup.no_changes", "无更改（跳过备份）");

    // Split view
    data.insert("split.select_note", "从左侧选择笔记");

    // Common buttons
    data.insert("common.confirm", "确认");
    data.insert("common.cancel", "取消");
    data.insert("common.close", "关闭");
    data.insert("common.loading", "加载中...");
    data.insert("common.error", "发生错误");
    data.insert("common.success", "成功");

    // Settings
    data.insert("settings.title", "设置");
    data.insert("settings.language", "语言");
    data.insert("settings.language_desc", "选择您喜欢的语言");
    data.insert("settings.stats", "统计");
    data.insert("settings.version", "版本");
    data.insert("settings.storage", "存储");
    data.insert("settings.about", "关于");

    // Knowledge Graph & Footer
    data.insert("home.knowledge_graph", "知识图谱");
    data.insert("app.footer", "知识属于每个人");

    // === USB ===
    data.insert("usb.title", "USB同步");
    data.insert("usb.scan", "扫描");
    data.insert("usb.scanning", "正在搜索USB设备");
    data.insert("usb.no_usb", "未检测到Lazarus USB");
    data.insert("usb.no_usb_hint", "插入带有lazarus.sync的USB或在下方初始化");
    data.insert("usb.error", "扫描失败");
    data.insert("usb.init_title", "初始化USB");
    data.insert("usb.init_desc", "创建新的Lazarus USB用于离线共享");
    data.insert("usb.init_btn", "初始化");
    data.insert("usb.init_error", "初始化失败");
    data.insert("usb.enter_path", "输入USB路径");
    data.insert("usb.notes", "笔记");
    data.insert("usb.posts", "帖子");
    data.insert("usb.packages", "软件包");
    data.insert("usb.sync", "同步");
    data.insert("usb.export", "导出");
    data.insert("usb.import", "导入");
    data.insert("home.usb_sync", "USB同步");
    data.insert("home.shortcut.wiki", "维基");
    data.insert("home.shortcut.usb", "USB");
    data.insert("home.shortcut.graph", "图表");
    data.insert("home.shortcut.review", "复习");

    // === Posts ===
    data.insert("posts.title", "帖子");
    data.insert("posts.new_post", "新帖子");
    data.insert("posts.no_posts", "暂无帖子");
    data.insert("posts.be_first", "成为第一个发帖的人！");
    data.insert("posts.author", "你的名字");
    data.insert("posts.title_placeholder", "标题");
    data.insert("posts.content_placeholder", "内容...");
    data.insert("posts.tags_placeholder", "标签（用逗号分隔）");
    data.insert("posts.post_btn", "发布");
    data.insert("posts.replies", "回复");
    data.insert("posts.delete_confirm", "删除这个帖子？");
    data.insert("posts.write_reply", "写回复...");
    data.insert("posts.reply_btn", "回复");
    // === Q&A ===
    data.insert("qna.title", "问答");
    data.insert("qna.ask_question", "提问");
    data.insert("qna.no_questions", "暂无问题");
    data.insert("qna.be_first", "成为第一个提问的人！");
    data.insert("qna.question_title", "问题标题");
    data.insert("qna.question_content", "描述你的问题...");
    data.insert("qna.post_question", "发布问题");
    data.insert("qna.answers", "回答");
    data.insert("qna.write_answer", "写你的回答...");
    data.insert("qna.post_answer", "发布回答");
    data.insert("qna.accept", "采纳");
    data.insert("qna.accepted", "已采纳");
    data.insert("qna.delete_confirm", "删除这个问题？");

    Translations::new(data)
}
