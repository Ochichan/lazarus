//! हिन्दी - Hindi translations

use super::Translations;
use std::collections::HashMap;

pub fn translations() -> Translations {
    let mut data = HashMap::new();

    // Common
    data.insert("app.name", "लाज़रस");
    data.insert("app.tagline", "ऑफ़लाइन व्यक्तिगत ज्ञान प्रबंधन");
    data.insert("nav.notes", "नोट्स");
    data.insert("nav.search", "खोजें");
    data.insert("nav.wiki", "विकी");

    // Home
    data.insert("home.notes", "नोट्स");
    data.insert("home.streak", "लगातार दिन");
    data.insert("home.day", "दिन");
    data.insert("home.quick_start", "त्वरित शुरुआत");
    data.insert("home.new_note", "नया नोट");
    data.insert("home.note_list", "नोट सूची");
    data.insert("home.split_view", "विभाजित दृश्य");
    data.insert("home.srs_review", "SRS समीक्षा");
    data.insert("home.search", "खोजें");
    data.insert("home.zim_manage", "ZIM प्रबंधन");
    data.insert("home.security", "सुरक्षा");
    data.insert("home.shortcuts", "कीबोर्ड शॉर्टकट");
    data.insert("home.shortcut.new_note", "नया नोट");
    data.insert("home.shortcut.search", "खोजें");
    data.insert("home.shortcut.help", "सहायता");

    // Editor
    data.insert("editor.title_placeholder", "शीर्षक दर्ज करें");
    data.insert("editor.content_placeholder", "यहाँ लिखना शुरू करें...");
    data.insert("editor.tags", "टैग");
    data.insert("editor.tags_placeholder", "टैग1, टैग2, ...");
    data.insert("editor.edit_note", "नोट संपादित करें");
    data.insert("editor.encrypt", "एन्क्रिप्शन टॉगल करें");
    data.insert("editor.focus", "फोकस");
    data.insert("editor.fullscreen", "पूर्ण स्क्रीन");
    data.insert("editor.save", "सहेजें");
    data.insert("editor.saved", "सहेजा गया");
    data.insert("editor.auto_saved", "स्वचालित सहेजा गया");
    data.insert("editor.changed", "बदला गया...");
    data.insert("editor.words", "शब्द");
    data.insert("editor.save_complete", "💾 सहेजा गया!");
    data.insert("editor.encrypt_on", "🔒 एन्क्रिप्शन सक्रिय");
    data.insert("editor.encrypt_off", "🔓 एन्क्रिप्शन निष्क्रिय");
    data.insert("editor.pin_required", "कृपया पहले PIN सेट करें (सुरक्षा मेनू)");
    data.insert("editor.pin_locked", "कृपया पहले PIN अनलॉक करें");

    // Notes list
    data.insert("notes.title", "नोट्स");
    data.insert("notes.export", "निर्यात");
    data.insert("notes.import", "आयात");
    data.insert("notes.no_notes", "अभी तक कोई नोट नहीं");
    data.insert("notes.create_first", "अपना पहला नोट बनाएं!");
    data.insert("notes.no_title", "बिना शीर्षक");
    data.insert("notes.edit", "संपादित");
    data.insert("notes.delete", "हटाएं");
    data.insert("notes.delete_confirm", "क्या आप वाकई हटाना चाहते हैं?");
    data.insert("notes.encrypted", "एन्क्रिप्टेड");
    data.insert("notes.created", "बनाया गया");
    data.insert("notes.updated", "अपडेट किया गया");
    data.insert("notes.find_duplicates", "डुप्लिकेट खोजें");
    data.insert("notes.select_all", "सभी चुनें");
    data.insert("notes.selected", "चयनित");
    data.insert("notes.create_package", "पैकेज बनाएं");
    data.insert("notes.duplicates_title", "डुप्लिकेट नोट्स");
    data.insert("notes.no_duplicates", "कोई डुप्लिकेट नोट नहीं मिला");
    data.insert("notes.export_package", "पैकेज निर्यात करें");
    data.insert("notes.import_package", "पैकेज आयात करें");
    data.insert("notes.package_title", "पैकेज शीर्षक");
    data.insert("notes.package_title_placeholder", "उदा., भौतिकी 101");
    data.insert("notes.package_author", "लेखक");
    data.insert("notes.package_author_placeholder", "आपका नाम");
    data.insert("notes.package_description", "विवरण");
    data.insert(
        "notes.package_description_placeholder",
        "पैकेज विवरण (वैकल्पिक)",
    );
    data.insert("notes.drop_file", ".laz फ़ाइल यहाँ छोड़ें");
    data.insert("notes.or", "या");
    data.insert("notes.select_file", "फ़ाइल चुनें");

    // Search
    data.insert("search.title", "खोजें");
    data.insert("search.placeholder", "खोज शब्द दर्ज करें...");
    data.insert("search.button", "खोजें");
    data.insert("search.results", "खोज परिणाम");
    data.insert("search.no_results", "कोई परिणाम नहीं मिला");
    data.insert("search.try_different", "अलग खोज शब्द आज़माएं");
    data.insert("search.tips", "खोज सुझाव");
    data.insert("search.tip1", "कई शब्द सभी शब्दों वाले परिणाम खोजते हैं");
    data.insert("search.tip2", "नोट्स और विकिपीडिया दोनों में खोजता है");
    data.insert("search.tip3", "शीर्षक मिलान पहले दिखाई देते हैं");

    // Wiki
    data.insert("wiki.search", "विकी खोजें");
    data.insert("wiki.recent_articles", "Recent Articles");
    data.insert("wiki.manage", "ZIM प्रबंधन");
    data.insert("wiki.status", "स्थिति");
    data.insert("wiki.loaded", "लोड किया गया");
    data.insert("wiki.directory", "ZIM डायरेक्टरी");
    data.insert(
        "wiki.directory_hint",
        ".zim फ़ाइलें इस फ़ोल्डर में कॉपी करें और रिफ्रेश करें",
    );
    data.insert("wiki.refresh", "रिफ्रेश");
    data.insert("wiki.open_folder", "फ़ोल्डर खोलें");
    data.insert("wiki.add", "ZIM जोड़ें");
    data.insert("wiki.add_placeholder", "ZIM फ़ाइल पथ दर्ज करें...");
    data.insert("wiki.loaded_files", "लोड की गई ZIM फ़ाइलें");
    data.insert("wiki.remove", "हटाएं");
    data.insert("wiki.no_zim", "कोई ZIM फ़ाइलें नहीं");
    data.insert(
        "wiki.no_zim_hint",
        "विकिपीडिया का उपयोग करने के लिए ZIM फ़ाइल जोड़ें",
    );
    data.insert("wiki.no_zim_loaded", "कोई ZIM फ़ाइलें लोड नहीं हुई");
    data.insert("wiki.loaded_zims", "लोड की गई ZIMs");
    data.insert("wiki.add_btn", "जोड़ें");
    data.insert("wiki.add_hint", "ZIM फ़ाइल का पूरा पथ दर्ज करें।");
    data.insert("wiki.name", "नाम");
    data.insert("wiki.path", "पथ");
    data.insert("wiki.action", "कार्रवाई");
    data.insert("wiki.zim_added", " ZIM जोड़ी गई: ");
    data.insert("wiki.no_new_zim", "कोई नई ZIM फ़ाइलें नहीं");
    data.insert("wiki.refresh_failed", "रिफ्रेश विफल");
    data.insert("wiki.enter_path", "कृपया पथ दर्ज करें");
    data.insert("wiki.add_failed", "जोड़ना विफल");
    data.insert("wiki.remove_confirm", "इस ZIM को हटाएं?");
    data.insert("wiki.zim_removed", "ZIM हटाई गई: ");
    data.insert("wiki.remove_failed", "हटाना विफल");
    data.insert("wiki.open_folder_msg", "कृपया फ़ाइल एक्सप्लोरर में फ़ोल्डर खोलें:");

    // SRS
    data.insert("srs.title", "SRS समीक्षा");
    data.insert("srs.cards", "कार्ड सूची");
    data.insert("srs.streak", "लगातार");
    data.insert("srs.show_answer", "उत्तर दिखाएं");
    data.insert("srs.again", "फिर से");
    data.insert("srs.hard", "कठिन");
    data.insert("srs.good", "अच्छा");
    data.insert("srs.easy", "आसान");
    data.insert("srs.complete", "🎉 आज की समीक्षा पूर्ण!");
    data.insert("srs.no_cards", "समीक्षा के लिए कोई कार्ड नहीं");
    data.insert("srs.progress", "कार्ड");
    data.insert("srs.today_review", "आज की समीक्षा");
    data.insert("srs.new_cards", "नया");
    data.insert("srs.learning", "सीख रहा है");
    data.insert("srs.mature", "परिपक्व");
    data.insert("srs.create_from_notes", "नोट्स से कार्ड बनाएं");
    data.insert("srs.create_cards", "कार्ड बनाएं");
    data.insert("srs.cards_created", "कार्ड बनाए गए");
    data.insert("srs.no_cards_extracted", "निकालने के लिए कोई कार्ड नहीं");
    data.insert("srs.repetitions", "दोहराव");
    data.insert("srs.streak_days", "लगातार");
    data.insert("srs.start_review", "समीक्षा शुरू करें");
    data.insert("srs.no_cards_yet", "अभी तक कोई कार्ड नहीं।");
    data.insert("srs.create_hint", "नोट खोलें और \"कार्ड बनाएं\" बटन क्लिक करें।");
    data.insert("srs.interval", "अंतराल");
    data.insert("srs.days", " दिन");
    data.insert("srs.delete_card_confirm", "इस कार्ड को हटाएं?");
    data.insert("srs.delete_failed", "हटाना विफल");

    // Security
    data.insert("security.title", "सुरक्षा सेटिंग्स");
    data.insert("security.pin_not_set", "PIN सेट नहीं है");
    data.insert("security.pin_enabled", "PIN सक्रिय");
    data.insert("security.locked", "लॉक - PIN दर्ज करें");
    data.insert("security.pin_input", "PIN दर्ज करें (6-32 अक्षर)");
    data.insert("security.set_pin", "PIN सेट करें");
    data.insert("security.remove_pin", "PIN हटाएं");
    data.insert("security.lock", "लॉक");
    data.insert("security.unlock", "अनलॉक");
    data.insert("security.wrong_pin", "गलत PIN");
    data.insert("security.pin_set_success", "PIN सेट हो गया");
    data.insert("security.pin_removed", "PIN हटा दिया गया");
    data.insert("security.unlocked", "अनलॉक हो गया");
    data.insert("security.pin_min_length", "PIN कम से कम 6 अक्षर होना चाहिए");
    data.insert("security.enter_current_pin", "वर्तमान PIN दर्ज करें");

    // Backup
    data.insert("backup.title", "बैकअप");
    data.insert("backup.info", "बैकअप जानकारी");
    data.insert("backup.now", "अभी बैकअप करें");
    data.insert("backup.complete", "बैकअप पूर्ण");
    data.insert("backup.no_changes", "कोई बदलाव नहीं (बैकअप छोड़ा गया)");

    // Split view
    data.insert("split.select_note", "बाईं ओर से नोट चुनें");

    // Common buttons
    data.insert("common.confirm", "पुष्टि करें");
    data.insert("common.cancel", "रद्द करें");
    data.insert("common.close", "बंद करें");
    data.insert("common.loading", "लोड हो रहा है...");
    data.insert("common.error", "एक त्रुटि हुई");
    data.insert("common.success", "सफल");

    // Settings
    data.insert("settings.title", "सेटिंग्स");
    data.insert("settings.language", "भाषा");
    data.insert("settings.language_desc", "अपनी पसंदीदा भाषा चुनें");
    data.insert("settings.stats", "आंकड़े");
    data.insert("settings.version", "संस्करण");
    data.insert("settings.storage", "स्टोरेज");
    data.insert("settings.about", "जानकारी");

    // Knowledge Graph & Footer
    data.insert("home.knowledge_graph", "ज्ञान ग्राफ");
    data.insert("app.footer", "सभी के लिए ज्ञान");

    // Knowledge Graph & Footer
    data.insert("home.knowledge_graph", "ज्ञान ग्राफ");
    data.insert("app.footer", "सभी के लिए ज्ञान");

    // === USB ===
    data.insert("usb.title", "USB सिंक");
    data.insert("usb.scan", "स्कैन");
    data.insert("usb.scanning", "USB डिवाइस खोज रहे हैं");
    data.insert("usb.no_usb", "कोई Lazarus USB नहीं मिला");
    data.insert(
        "usb.no_usb_hint",
        "lazarus.sync वाला USB डालें या नीचे इनिशियलाइज़ करें",
    );
    data.insert("usb.error", "स्कैन विफल");
    data.insert("usb.init_title", "USB इनिशियलाइज़");
    data.insert("usb.init_desc", "ऑफलाइन शेयरिंग के लिए नया Lazarus USB बनाएं");
    data.insert("usb.init_btn", "इनिशियलाइज़");
    data.insert("usb.init_error", "इनिशियलाइज़ विफल");
    data.insert("usb.enter_path", "USB पथ दर्ज करें");
    data.insert("usb.notes", "नोट्स");
    data.insert("usb.posts", "पोस्ट");
    data.insert("usb.packages", "पैकेज");
    data.insert("usb.sync", "सिंक");
    data.insert("usb.export", "एक्सपोर्ट");
    data.insert("usb.import", "इम्पोर्ट");
    data.insert("home.usb_sync", "USB सिंक");
    data.insert("home.shortcut.wiki", "विकी");
    data.insert("home.shortcut.usb", "USB");
    data.insert("home.shortcut.graph", "ग्राफ");
    data.insert("home.shortcut.review", "रिव्यू");
    Translations::new(data)
}
