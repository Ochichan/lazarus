//! English translations

use super::Translations;
use std::collections::HashMap;

pub fn translations() -> Translations {
    let mut data = HashMap::new();

    // Common
    data.insert("app.name", "Lazarus");
    data.insert("app.tagline", "Offline Personal Knowledge Management");
    data.insert("nav.notes", "Notes");
    data.insert("nav.search", "Search");
    data.insert("nav.wiki", "Wiki");

    // Home
    data.insert("home.notes", "Notes");
    data.insert("home.streak", "Study Streak");
    data.insert("home.day", "day(s)");
    data.insert("home.quick_start", "Quick Start");
    data.insert("home.new_note", "New Note");
    data.insert("home.note_list", "Note List");
    data.insert("home.split_view", "Split View");
    data.insert("home.srs_review", "SRS Review");
    data.insert("home.search", "Search");
    data.insert("home.zim_manage", "ZIM Manage");
    data.insert("home.security", "Security");
    data.insert("home.shortcuts", "Keyboard Shortcuts");
    data.insert("home.shortcut.new_note", "New note");
    data.insert("home.shortcut.search", "Search");
    data.insert("home.shortcut.help", "Help");

    // Editor
    data.insert("editor.title_placeholder", "Enter title");
    data.insert("editor.content_placeholder", "Start writing here...");
    data.insert("editor.tags", "Tags");
    data.insert("editor.tags_placeholder", "tag1, tag2, ...");
    data.insert("editor.edit_note", "Edit Note");
    data.insert("editor.encrypt", "Toggle encryption");
    data.insert("editor.focus", "Focus");
    data.insert("editor.fullscreen", "Fullscreen");
    data.insert("editor.save", "Save");
    data.insert("editor.saved", "Saved");
    data.insert("editor.auto_saved", "Auto saved");
    data.insert("editor.changed", "Changed...");
    data.insert("editor.words", "words");
    data.insert("editor.save_complete", "💾 Saved!");
    data.insert("editor.encrypt_on", "🔒 Encryption enabled");
    data.insert("editor.encrypt_off", "🔓 Encryption disabled");
    data.insert(
        "editor.pin_required",
        "Please set PIN first (Security menu)",
    );
    data.insert("editor.pin_locked", "Please unlock PIN first");

    // Notes list
    data.insert("notes.title", "Notes");
    data.insert("notes.export", "Export");
    data.insert("notes.import", "Import");
    data.insert("notes.no_notes", "No notes yet");
    data.insert("notes.create_first", "Create your first note!");
    data.insert("notes.no_title", "Untitled");
    data.insert("notes.edit", "Edit");
    data.insert("notes.delete", "Delete");
    data.insert("notes.delete_confirm", "Are you sure you want to delete?");
    data.insert("notes.encrypted", "Encrypted");
    data.insert("notes.created", "Created");
    data.insert("notes.updated", "Updated");
    data.insert("notes.find_duplicates", "Find Duplicates");
    data.insert("notes.select_all", "Select All");
    data.insert("notes.selected", "selected");
    data.insert("notes.create_package", "Create Package");
    data.insert("notes.duplicates_title", "Duplicate Notes");
    data.insert("notes.no_duplicates", "No duplicate notes found");
    data.insert("notes.export_package", "Export Package");
    data.insert("notes.import_package", "Import Package");
    data.insert("notes.package_title", "Package Title");
    data.insert("notes.package_title_placeholder", "e.g., Physics 101");
    data.insert("notes.package_author", "Author");
    data.insert("notes.package_author_placeholder", "Your name");
    data.insert("notes.package_description", "Description");
    data.insert(
        "notes.package_description_placeholder",
        "Package description (optional)",
    );
    data.insert("notes.drop_file", "Drop .laz file here");
    data.insert("notes.or", "or");
    data.insert("notes.select_file", "Select File");

    // Search
    data.insert("search.title", "Search");
    data.insert("search.placeholder", "Enter search term...");
    data.insert("search.button", "Search");
    data.insert("search.results", "Search Results");
    data.insert("search.no_results", "No results found");
    data.insert("search.try_different", "Try a different search term");
    data.insert("search.tips", "Search Tips");
    data.insert(
        "search.tip1",
        "Multiple words search for results containing all terms",
    );
    data.insert("search.tip2", "Searches both notes and Wikipedia");
    data.insert("search.tip3", "Title matches appear first");

    // Wiki
    data.insert("wiki.search", "Wiki Search");
    data.insert("wiki.recent_articles", "Recent Articles");
    data.insert("wiki.manage", "ZIM Management");
    data.insert("wiki.status", "Status");
    data.insert("wiki.loaded", "loaded");
    data.insert("wiki.directory", "ZIM Directory");
    data.insert(
        "wiki.directory_hint",
        "Copy .zim files to this folder and refresh",
    );
    data.insert("wiki.refresh", "Refresh");
    data.insert("wiki.open_folder", "Open Folder");
    data.insert("wiki.add", "Add ZIM");
    data.insert("wiki.add_placeholder", "Enter ZIM file path...");
    data.insert("wiki.loaded_files", "Loaded ZIM Files");
    data.insert("wiki.remove", "Remove");
    data.insert("wiki.no_zim", "No ZIM files");
    data.insert("wiki.no_zim_hint", "Add a ZIM file to use Wikipedia");
    data.insert("wiki.no_zim_loaded", "No ZIM files loaded");
    data.insert("wiki.loaded_zims", "Loaded ZIMs");
    data.insert("wiki.add_btn", "Add");
    data.insert("wiki.add_hint", "Enter the full path to ZIM file.");
    data.insert("wiki.name", "Name");
    data.insert("wiki.path", "Path");
    data.insert("wiki.action", "Action");
    data.insert("wiki.zim_added", " ZIM(s) added: ");
    data.insert("wiki.no_new_zim", "No new ZIM files");
    data.insert("wiki.refresh_failed", "Refresh failed");
    data.insert("wiki.enter_path", "Please enter a path");
    data.insert("wiki.add_failed", "Add failed");
    data.insert("wiki.remove_confirm", "Remove this ZIM?");
    data.insert("wiki.zim_removed", "ZIM removed: ");
    data.insert("wiki.remove_failed", "Remove failed");
    data.insert(
        "wiki.open_folder_msg",
        "Please open the folder in file explorer:",
    );

    // SRS
    data.insert("srs.title", "SRS Review");
    data.insert("srs.cards", "Card List");
    data.insert("srs.streak", "streak");
    data.insert("srs.show_answer", "Show Answer");
    data.insert("srs.again", "Again");
    data.insert("srs.hard", "Hard");
    data.insert("srs.good", "Good");
    data.insert("srs.easy", "Easy");
    data.insert("srs.complete", "🎉 Today's review complete!");
    data.insert("srs.no_cards", "No cards to review");
    data.insert("srs.progress", "cards");
    data.insert("srs.today_review", "Due Today");
    data.insert("srs.new_cards", "New");
    data.insert("srs.learning", "Learning");
    data.insert("srs.mature", "Mature");
    data.insert("srs.create_from_notes", "Create cards from notes");
    data.insert("srs.create_cards", "Create Cards");
    data.insert("srs.cards_created", "cards created");
    data.insert("srs.no_cards_extracted", "No cards to extract");
    data.insert("srs.repetitions", "Reps");
    data.insert("srs.streak_days", "Streak");
    data.insert("srs.start_review", "Start Review");
    data.insert("srs.no_cards_yet", "No cards yet.");
    data.insert(
        "srs.create_hint",
        "Open a note and click \"Create Cards\" button.",
    );
    data.insert("srs.interval", "Interval");
    data.insert("srs.days", " days");
    data.insert("srs.delete_card_confirm", "Delete this card?");
    data.insert("srs.delete_failed", "Delete failed");

    // Security
    data.insert("security.title", "Security Settings");
    data.insert("security.pin_not_set", "PIN not set");
    data.insert("security.pin_enabled", "PIN enabled");
    data.insert("security.locked", "Locked - Enter PIN");
    data.insert("security.pin_input", "Enter PIN (6-32 characters)");
    data.insert("security.set_pin", "Set PIN");
    data.insert("security.remove_pin", "Remove PIN");
    data.insert("security.lock", "Lock");
    data.insert("security.unlock", "Unlock");
    data.insert("security.wrong_pin", "Wrong PIN");
    data.insert("security.pin_set_success", "PIN has been set");
    data.insert("security.pin_removed", "PIN has been removed");
    data.insert("security.unlocked", "Unlocked");
    data.insert(
        "security.pin_min_length",
        "PIN must be at least 6 characters",
    );
    data.insert("security.enter_current_pin", "Enter current PIN");

    // Backup
    data.insert("backup.title", "Backup");
    data.insert("backup.info", "Backup Info");
    data.insert("backup.now", "Backup Now");
    data.insert("backup.complete", "Backup complete");
    data.insert("backup.no_changes", "No changes (backup skipped)");

    // Split view
    data.insert("split.select_note", "Select a note from the left");

    // Common buttons
    data.insert("common.confirm", "Confirm");
    data.insert("common.cancel", "Cancel");
    data.insert("common.close", "Close");
    data.insert("common.loading", "Loading...");
    data.insert("common.error", "An error occurred");
    data.insert("common.success", "Success");

    // Settings
    data.insert("settings.title", "Settings");
    data.insert("settings.language", "Language");
    data.insert("settings.language_desc", "Select your preferred language");
    data.insert("settings.stats", "Statistics");
    data.insert("settings.version", "Version");
    data.insert("settings.storage", "Storage");
    data.insert("settings.about", "About");

    // Knowledge Graph & Footer
    data.insert("home.knowledge_graph", "Knowledge Graph");
    data.insert("app.footer", "Knowledge for All");

    // === USB ===
    data.insert("usb.title", "USB Sync");
    data.insert("usb.scan", "Scan");
    data.insert("usb.scanning", "Scanning for USB devices");
    data.insert("usb.no_usb", "No Lazarus USB detected");
    data.insert(
        "usb.no_usb_hint",
        "Insert a USB with lazarus.sync or initialize below",
    );
    data.insert("usb.error", "Failed to scan");
    data.insert("usb.init_title", "Initialize USB");
    data.insert(
        "usb.init_desc",
        "Create a new Lazarus USB for offline sharing",
    );
    data.insert("usb.init_btn", "Initialize");
    data.insert("usb.init_error", "Failed to initialize");
    data.insert("usb.enter_path", "Enter USB path");
    data.insert("usb.notes", "Notes");
    data.insert("usb.posts", "Posts");
    data.insert("usb.packages", "Packages");
    data.insert("usb.sync", "Sync");
    data.insert("usb.export", "Export");
    data.insert("usb.import", "Import");

    data.insert("home.usb_sync", "USB Sync");
    data.insert("home.shortcut.wiki", "Wiki");
    data.insert("home.shortcut.usb", "USB Sync");
    data.insert("home.shortcut.graph", "Graph");
    data.insert("home.shortcut.review", "Review");
    Translations::new(data)
}
