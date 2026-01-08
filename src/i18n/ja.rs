//! 日本語 - Japanese translations

use super::Translations;
use std::collections::HashMap;

pub fn translations() -> Translations {
    let mut data = HashMap::new();

    // Common
    data.insert("app.name", "Lazarus");
    data.insert("app.tagline", "オフライン個人知識管理");
    data.insert("nav.notes", "ノート");
    data.insert("nav.search", "検索");
    data.insert("nav.wiki", "Wiki");

    // Home
    data.insert("home.notes", "ノート");
    data.insert("home.streak", "連続学習");
    data.insert("home.day", "日");
    data.insert("home.quick_start", "クイックスタート");
    data.insert("home.new_note", "新しいノート");
    data.insert("home.note_list", "ノート一覧");
    data.insert("home.split_view", "分割表示");
    data.insert("home.srs_review", "SRS復習");
    data.insert("home.search", "検索");
    data.insert("home.zim_manage", "ZIM管理");
    data.insert("home.security", "セキュリティ");
    data.insert("home.shortcuts", "キーボードショートカット");
    data.insert("home.shortcut.new_note", "新しいノート");
    data.insert("home.shortcut.search", "検索");
    data.insert("home.shortcut.help", "ヘルプ");

    // Editor
    data.insert("editor.title_placeholder", "タイトルを入力");
    data.insert("editor.content_placeholder", "ここに書き始める...");
    data.insert("editor.tags", "タグ");
    data.insert("editor.tags_placeholder", "タグ1, タグ2, ...");
    data.insert("editor.edit_note", "ノートを編集");
    data.insert("editor.encrypt", "暗号化切替");
    data.insert("editor.focus", "集中");
    data.insert("editor.fullscreen", "全画面");
    data.insert("editor.save", "保存");
    data.insert("editor.saved", "保存済み");
    data.insert("editor.auto_saved", "自動保存済み");
    data.insert("editor.changed", "変更あり...");
    data.insert("editor.words", "単語");
    data.insert("editor.save_complete", "💾 保存完了!");
    data.insert("editor.encrypt_on", "🔒 暗号化有効");
    data.insert("editor.encrypt_off", "🔓 暗号化無効");
    data.insert(
        "editor.pin_required",
        "まずPINを設定してください（セキュリティメニュー）",
    );
    data.insert("editor.pin_locked", "まずPINを解除してください");

    // Notes list
    data.insert("notes.title", "ノート");
    data.insert("notes.export", "エクスポート");
    data.insert("notes.import", "インポート");
    data.insert("notes.no_notes", "ノートがありません");
    data.insert("notes.create_first", "最初のノートを作成しましょう！");
    data.insert("notes.no_title", "無題");
    data.insert("notes.edit", "編集");
    data.insert("notes.delete", "削除");
    data.insert("notes.delete_confirm", "本当に削除しますか？");
    data.insert("notes.encrypted", "暗号化済み");
    data.insert("notes.created", "作成日");
    data.insert("notes.updated", "更新日");
    data.insert("notes.find_duplicates", "重複を検索");
    data.insert("notes.select_all", "すべて選択");
    data.insert("notes.selected", "件選択中");
    data.insert("notes.create_package", "パッケージ作成");
    data.insert("notes.duplicates_title", "重複ノート");
    data.insert("notes.no_duplicates", "重複ノートはありません");
    data.insert("notes.export_package", "パッケージをエクスポート");
    data.insert("notes.import_package", "パッケージをインポート");
    data.insert("notes.package_title", "パッケージタイトル");
    data.insert("notes.package_title_placeholder", "例：物理学 101");
    data.insert("notes.package_author", "作成者");
    data.insert("notes.package_author_placeholder", "あなたの名前");
    data.insert("notes.package_description", "説明");
    data.insert(
        "notes.package_description_placeholder",
        "パッケージの説明（任意）",
    );
    data.insert("notes.drop_file", ".lazファイルをここにドロップ");
    data.insert("notes.or", "または");
    data.insert("notes.select_file", "ファイルを選択");

    // Search
    data.insert("search.title", "検索");
    data.insert("search.placeholder", "検索キーワードを入力...");
    data.insert("search.button", "検索");
    data.insert("search.results", "検索結果");
    data.insert("search.no_results", "結果が見つかりません");
    data.insert("search.try_different", "別のキーワードを試してください");
    data.insert("search.tips", "検索のヒント");
    data.insert("search.tip1", "複数の単語はすべてを含む結果を検索します");
    data.insert("search.tip2", "ノートとWikipediaの両方を検索します");
    data.insert("search.tip3", "タイトルの一致が最初に表示されます");

    // Wiki
    data.insert("wiki.search", "Wiki検索");
    data.insert("wiki.recent_articles", "Recent Articles");
    data.insert("wiki.manage", "ZIM管理");
    data.insert("wiki.status", "状態");
    data.insert("wiki.loaded", "読み込み済み");
    data.insert("wiki.directory", "ZIMディレクトリ");
    data.insert(
        "wiki.directory_hint",
        ".zimファイルをこのフォルダにコピーして更新",
    );
    data.insert("wiki.refresh", "更新");
    data.insert("wiki.open_folder", "フォルダを開く");
    data.insert("wiki.add", "ZIMを追加");
    data.insert("wiki.add_placeholder", "ZIMファイルのパスを入力...");
    data.insert("wiki.loaded_files", "読み込み済みZIMファイル");
    data.insert("wiki.remove", "削除");
    data.insert("wiki.no_zim", "ZIMファイルなし");
    data.insert(
        "wiki.no_zim_hint",
        "WikipediaのZIMファイルを追加してください",
    );
    data.insert("wiki.no_zim_loaded", "ZIMファイルが読み込まれていません");
    data.insert("wiki.loaded_zims", "読み込み済みZIM");
    data.insert("wiki.add_btn", "追加");
    data.insert("wiki.add_hint", "ZIMファイルのフルパスを入力してください。");
    data.insert("wiki.name", "名前");
    data.insert("wiki.path", "パス");
    data.insert("wiki.action", "操作");
    data.insert("wiki.zim_added", " ZIM追加: ");
    data.insert("wiki.no_new_zim", "新しいZIMファイルなし");
    data.insert("wiki.refresh_failed", "更新に失敗");
    data.insert("wiki.enter_path", "パスを入力してください");
    data.insert("wiki.add_failed", "追加に失敗");
    data.insert("wiki.remove_confirm", "このZIMを削除しますか？");
    data.insert("wiki.zim_removed", "ZIM削除: ");
    data.insert("wiki.remove_failed", "削除に失敗");
    data.insert(
        "wiki.open_folder_msg",
        "エクスプローラーでフォルダを開いてください:",
    );

    // SRS
    data.insert("srs.title", "SRS復習");
    data.insert("srs.cards", "カード一覧");
    data.insert("srs.streak", "連続");
    data.insert("srs.show_answer", "答えを表示");
    data.insert("srs.again", "もう一度");
    data.insert("srs.hard", "難しい");
    data.insert("srs.good", "良い");
    data.insert("srs.easy", "簡単");
    data.insert("srs.complete", "🎉 今日の復習完了!");
    data.insert("srs.no_cards", "復習するカードがありません");
    data.insert("srs.progress", "カード");
    data.insert("srs.today_review", "今日の復習");
    data.insert("srs.new_cards", "新規");
    data.insert("srs.learning", "学習中");
    data.insert("srs.mature", "習得済み");
    data.insert("srs.create_from_notes", "ノートからカードを作成");
    data.insert("srs.create_cards", "カード作成");
    data.insert("srs.cards_created", "枚のカードが作成されました");
    data.insert("srs.no_cards_extracted", "抽出するカードがありません");
    data.insert("srs.repetitions", "反復");
    data.insert("srs.streak_days", "連続");
    data.insert("srs.start_review", "復習を開始");
    data.insert("srs.no_cards_yet", "まだカードがありません。");
    data.insert(
        "srs.create_hint",
        "ノートを開いて「カード作成」ボタンをクリックしてください。",
    );
    data.insert("srs.interval", "間隔");
    data.insert("srs.days", "日");
    data.insert("srs.delete_card_confirm", "このカードを削除しますか？");
    data.insert("srs.delete_failed", "削除に失敗");

    // Security
    data.insert("security.title", "セキュリティ設定");
    data.insert("security.pin_not_set", "PINが設定されていません");
    data.insert("security.pin_enabled", "PIN有効");
    data.insert("security.locked", "ロック中 - PINを入力");
    data.insert("security.pin_input", "PINを入力（6-32文字）");
    data.insert("security.set_pin", "PINを設定");
    data.insert("security.remove_pin", "PINを削除");
    data.insert("security.lock", "ロック");
    data.insert("security.unlock", "解除");
    data.insert("security.wrong_pin", "PINが違います");
    data.insert("security.pin_set_success", "PINが設定されました");
    data.insert("security.pin_removed", "PINが削除されました");
    data.insert("security.unlocked", "ロック解除されました");
    data.insert("security.pin_min_length", "PINは6文字以上必要です");
    data.insert("security.enter_current_pin", "現在のPINを入力");

    // Backup
    data.insert("backup.title", "バックアップ");
    data.insert("backup.info", "バックアップ情報");
    data.insert("backup.now", "今すぐバックアップ");
    data.insert("backup.complete", "バックアップ完了");
    data.insert("backup.no_changes", "変更なし（バックアップスキップ）");

    // Split view
    data.insert("split.select_note", "左からノートを選択してください");

    // Common buttons
    data.insert("common.confirm", "確認");
    data.insert("common.cancel", "キャンセル");
    data.insert("common.close", "閉じる");
    data.insert("common.loading", "読み込み中...");
    data.insert("common.error", "エラーが発生しました");
    data.insert("common.success", "成功");

    // Settings
    data.insert("settings.title", "設定");
    data.insert("settings.language", "言語");
    data.insert("settings.language_desc", "ご希望の言語を選択してください");
    data.insert("settings.stats", "統計");
    data.insert("settings.version", "バージョン");
    data.insert("settings.storage", "ストレージ");
    data.insert("settings.about", "このアプリについて");

    // Knowledge Graph & Footer
    data.insert("home.knowledge_graph", "ナレッジグラフ");
    data.insert("app.footer", "すべての人に知識を");

    // === USB ===
    data.insert("usb.title", "USB同期");
    data.insert("usb.scan", "スキャン");
    data.insert("usb.scanning", "USBデバイスを検索中");
    data.insert("usb.no_usb", "Lazarus USBが検出されません");
    data.insert(
        "usb.no_usb_hint",
        "lazarus.syncファイルのあるUSBを挿入するか、以下で初期化してください",
    );
    data.insert("usb.error", "スキャン失敗");
    data.insert("usb.init_title", "USB初期化");
    data.insert("usb.init_desc", "オフライン共有用の新しいLazarus USBを作成");
    data.insert("usb.init_btn", "初期化");
    data.insert("usb.init_error", "初期化失敗");
    data.insert("usb.enter_path", "USBパスを入力");
    data.insert("usb.notes", "ノート");
    data.insert("usb.posts", "投稿");
    data.insert("usb.packages", "パッケージ");
    data.insert("usb.sync", "同期");
    data.insert("usb.export", "エクスポート");
    data.insert("usb.import", "インポート");
    data.insert("home.usb_sync", "USB同期");
    data.insert("home.shortcut.wiki", "ウィキ");
    data.insert("home.shortcut.usb", "USB");
    data.insert("home.shortcut.graph", "グラフ");
    data.insert("home.shortcut.review", "復習");

    // === Posts ===
    data.insert("posts.title", "投稿");
    data.insert("posts.new_post", "新規投稿");
    data.insert("posts.no_posts", "投稿がありません");
    data.insert("posts.be_first", "最初の投稿をしましょう！");
    data.insert("posts.author", "名前");
    data.insert("posts.title_placeholder", "タイトル");
    data.insert("posts.content_placeholder", "内容...");
    data.insert("posts.tags_placeholder", "タグ（カンマ区切り）");
    data.insert("posts.post_btn", "投稿");
    data.insert("posts.replies", "返信");
    data.insert("posts.delete_confirm", "この投稿を削除しますか？");
    data.insert("posts.write_reply", "返信を書く...");
    data.insert("posts.reply_btn", "返信");
    // === Q&A ===
    data.insert("qna.title", "Q&A");
    data.insert("qna.ask_question", "質問する");
    data.insert("qna.no_questions", "質問がありません");
    data.insert("qna.be_first", "最初の質問をしましょう！");
    data.insert("qna.question_title", "質問のタイトル");
    data.insert("qna.question_content", "質問の内容を書いてください...");
    data.insert("qna.post_question", "質問を投稿");
    data.insert("qna.answers", "回答");
    data.insert("qna.write_answer", "回答を書く...");
    data.insert("qna.post_answer", "回答を投稿");
    data.insert("qna.accept", "採用");
    data.insert("qna.accepted", "採用済み");
    data.insert("qna.delete_confirm", "この質問を削除しますか？");

    Translations::new(data)
}
