//! Kiswahili - Swahili translations

use super::Translations;
use std::collections::HashMap;

pub fn translations() -> Translations {
    let mut data = HashMap::new();

    // Common
    data.insert("app.name", "Lazarus");
    data.insert("app.tagline", "Usimamizi wa Maarifa Binafsi bila Mtandao");
    data.insert("nav.notes", "Madokezo");
    data.insert("nav.search", "Tafuta");
    data.insert("nav.wiki", "Wiki");

    // Home
    data.insert("home.notes", "Madokezo");
    data.insert("home.streak", "Siku za Kujifunza");
    data.insert("home.day", "siku");
    data.insert("home.quick_start", "Anza Haraka");
    data.insert("home.new_note", "Dokezo Jipya");
    data.insert("home.note_list", "Orodha ya Madokezo");
    data.insert("home.split_view", "Mwonekano Uliogawanywa");
    data.insert("home.srs_review", "Mapitio ya SRS");
    data.insert("home.search", "Tafuta");
    data.insert("home.zim_manage", "Simamia ZIM");
    data.insert("home.security", "Usalama");
    data.insert("home.shortcuts", "Njia za Mkato za Kibodi");
    data.insert("home.shortcut.new_note", "Dokezo jipya");
    data.insert("home.shortcut.search", "Tafuta");
    data.insert("home.shortcut.help", "Msaada");

    // Editor
    data.insert("editor.title_placeholder", "Ingiza kichwa");
    data.insert("editor.content_placeholder", "Anza kuandika hapa...");
    data.insert("editor.tags", "Vitambulisho");
    data.insert("editor.tags_placeholder", "tag1, tag2, ...");
    data.insert("editor.edit_note", "Hariri Dokezo");
    data.insert("editor.encrypt", "Badilisha usimbaji");
    data.insert("editor.focus", "Zingatia");
    data.insert("editor.fullscreen", "Skrini Kamili");
    data.insert("editor.save", "Hifadhi");
    data.insert("editor.saved", "Imehifadhiwa");
    data.insert("editor.auto_saved", "Imehifadhiwa kiotomatiki");
    data.insert("editor.changed", "Imebadilishwa...");
    data.insert("editor.words", "maneno");
    data.insert("editor.save_complete", "💾 Imehifadhiwa!");
    data.insert("editor.encrypt_on", "🔒 Usimbaji umewashwa");
    data.insert("editor.encrypt_off", "🔓 Usimbaji umezimwa");
    data.insert(
        "editor.pin_required",
        "Tafadhali weka PIN kwanza (Menyu ya Usalama)",
    );
    data.insert("editor.pin_locked", "Tafadhali fungua PIN kwanza");

    // Notes list
    data.insert("notes.title", "Madokezo");
    data.insert("notes.export", "Hamisha");
    data.insert("notes.import", "Ingiza");
    data.insert("notes.no_notes", "Hakuna madokezo bado");
    data.insert("notes.create_first", "Unda dokezo lako la kwanza!");
    data.insert("notes.no_title", "Bila Kichwa");
    data.insert("notes.edit", "Hariri");
    data.insert("notes.delete", "Futa");
    data.insert("notes.delete_confirm", "Una uhakika unataka kufuta?");
    data.insert("notes.encrypted", "Imesimbwa");
    data.insert("notes.created", "Imeundwa");
    data.insert("notes.updated", "Imesasishwa");
    data.insert("notes.find_duplicates", "Tafuta Nakala");
    data.insert("notes.select_all", "Chagua Zote");
    data.insert("notes.selected", "zimechaguliwa");
    data.insert("notes.create_package", "Unda Kifurushi");
    data.insert("notes.duplicates_title", "Madokezo Yanayorudiwa");
    data.insert("notes.no_duplicates", "Hakuna madokezo yanayorudiwa");
    data.insert("notes.export_package", "Hamisha Kifurushi");
    data.insert("notes.import_package", "Ingiza Kifurushi");
    data.insert("notes.package_title", "Kichwa cha Kifurushi");
    data.insert("notes.package_title_placeholder", "mfano, Fizikia 101");
    data.insert("notes.package_author", "Mwandishi");
    data.insert("notes.package_author_placeholder", "Jina lako");
    data.insert("notes.package_description", "Maelezo");
    data.insert(
        "notes.package_description_placeholder",
        "Maelezo ya kifurushi (si lazima)",
    );
    data.insert("notes.drop_file", "Dondosha faili ya .laz hapa");
    data.insert("notes.or", "au");
    data.insert("notes.select_file", "Chagua Faili");

    // Search
    data.insert("search.title", "Tafuta");
    data.insert("search.placeholder", "Ingiza neno la kutafuta...");
    data.insert("search.button", "Tafuta");
    data.insert("search.results", "Matokeo ya Utafutaji");
    data.insert("search.no_results", "Hakuna matokeo yaliyopatikana");
    data.insert("search.try_different", "Jaribu neno tofauti la kutafuta");
    data.insert("search.tips", "Vidokezo vya Utafutaji");
    data.insert(
        "search.tip1",
        "Maneno mengi hutafuta matokeo yenye maneno yote",
    );
    data.insert("search.tip2", "Inatafuta madokezo na Wikipedia");
    data.insert("search.tip3", "Mechi za kichwa zinaonekana kwanza");

    // Wiki
    data.insert("wiki.search", "Tafuta Wiki");
    data.insert("wiki.recent_articles", "Recent Articles");
    data.insert("wiki.manage", "Simamia ZIM");
    data.insert("wiki.status", "Hali");
    data.insert("wiki.loaded", "zimepakiwa");
    data.insert("wiki.directory", "Saraka ya ZIM");
    data.insert(
        "wiki.directory_hint",
        "Nakili faili za .zim kwenye folda hii na uonyeshe upya",
    );
    data.insert("wiki.refresh", "Onyesha Upya");
    data.insert("wiki.open_folder", "Fungua Folda");
    data.insert("wiki.add", "Ongeza ZIM");
    data.insert("wiki.add_placeholder", "Ingiza njia ya faili ya ZIM...");
    data.insert("wiki.loaded_files", "Faili za ZIM Zilizopakiwa");
    data.insert("wiki.remove", "Ondoa");
    data.insert("wiki.no_zim", "Hakuna faili za ZIM");
    data.insert("wiki.no_zim_hint", "Ongeza faili ya ZIM kutumia Wikipedia");
    data.insert("wiki.no_zim_loaded", "Hakuna faili za ZIM zilizopakiwa");
    data.insert("wiki.loaded_zims", "ZIM Zilizopakiwa");
    data.insert("wiki.add_btn", "Ongeza");
    data.insert("wiki.add_hint", "Ingiza njia kamili ya faili ya ZIM.");
    data.insert("wiki.name", "Jina");
    data.insert("wiki.path", "Njia");
    data.insert("wiki.action", "Kitendo");
    data.insert("wiki.zim_added", " ZIM zimeongezwa: ");
    data.insert("wiki.no_new_zim", "Hakuna faili mpya za ZIM");
    data.insert("wiki.refresh_failed", "Kushindwa kuonyesha upya");
    data.insert("wiki.enter_path", "Tafadhali ingiza njia");
    data.insert("wiki.add_failed", "Kushindwa kuongeza");
    data.insert("wiki.remove_confirm", "Ondoa ZIM hii?");
    data.insert("wiki.zim_removed", "ZIM imeondolewa: ");
    data.insert("wiki.remove_failed", "Kushindwa kuondoa");
    data.insert(
        "wiki.open_folder_msg",
        "Tafadhali fungua folda katika kivinjari cha faili:",
    );

    // SRS
    data.insert("srs.title", "Mapitio ya SRS");
    data.insert("srs.cards", "Orodha ya Kadi");
    data.insert("srs.streak", "mfululizo");
    data.insert("srs.show_answer", "Onyesha Jibu");
    data.insert("srs.again", "Tena");
    data.insert("srs.hard", "Ngumu");
    data.insert("srs.good", "Nzuri");
    data.insert("srs.easy", "Rahisi");
    data.insert("srs.complete", "🎉 Mapitio ya leo yamekamilika!");
    data.insert("srs.no_cards", "Hakuna kadi za kupitia");
    data.insert("srs.progress", "kadi");
    data.insert("srs.today_review", "Mapitio ya Leo");
    data.insert("srs.new_cards", "Mpya");
    data.insert("srs.learning", "Zinajifunzwa");
    data.insert("srs.mature", "Zimekomaa");
    data.insert("srs.create_from_notes", "Unda kadi kutoka madokezo");
    data.insert("srs.create_cards", "Unda Kadi");
    data.insert("srs.cards_created", "kadi zimeundwa");
    data.insert("srs.no_cards_extracted", "Hakuna kadi za kuchimba");
    data.insert("srs.repetitions", "Marudio");
    data.insert("srs.streak_days", "Mfululizo");
    data.insert("srs.start_review", "Anza Mapitio");
    data.insert("srs.no_cards_yet", "Hakuna kadi bado.");
    data.insert(
        "srs.create_hint",
        "Fungua dokezo na ubofye kitufe cha \"Unda Kadi\".",
    );
    data.insert("srs.interval", "Muda");
    data.insert("srs.days", " siku");
    data.insert("srs.delete_card_confirm", "Futa kadi hii?");
    data.insert("srs.delete_failed", "Kushindwa kufuta");

    // Security
    data.insert("security.title", "Mipangilio ya Usalama");
    data.insert("security.pin_not_set", "PIN haijawekwa");
    data.insert("security.pin_enabled", "PIN imewashwa");
    data.insert("security.locked", "Imefungwa - Ingiza PIN");
    data.insert("security.pin_input", "Ingiza PIN (herufi 6-32)");
    data.insert("security.set_pin", "Weka PIN");
    data.insert("security.remove_pin", "Ondoa PIN");
    data.insert("security.lock", "Funga");
    data.insert("security.unlock", "Fungua");
    data.insert("security.wrong_pin", "PIN mbaya");
    data.insert("security.pin_set_success", "PIN imewekwa");
    data.insert("security.pin_removed", "PIN imeondolewa");
    data.insert("security.unlocked", "Imefunguliwa");
    data.insert(
        "security.pin_min_length",
        "PIN lazima iwe na herufi 6 au zaidi",
    );
    data.insert("security.enter_current_pin", "Ingiza PIN ya sasa");

    // Backup
    data.insert("backup.title", "Hifadhi Rudufu");
    data.insert("backup.info", "Taarifa za Hifadhi Rudufu");
    data.insert("backup.now", "Hifadhi Rudufu Sasa");
    data.insert("backup.complete", "Hifadhi rudufu imekamilika");
    data.insert(
        "backup.no_changes",
        "Hakuna mabadiliko (hifadhi rudufu imerukwa)",
    );

    // Split view
    data.insert("split.select_note", "Chagua dokezo kutoka kushoto");

    // Common buttons
    data.insert("common.confirm", "Thibitisha");
    data.insert("common.cancel", "Ghairi");
    data.insert("common.close", "Funga");
    data.insert("common.loading", "Inapakia...");
    data.insert("common.error", "Hitilafu imetokea");
    data.insert("common.success", "Mafanikio");

    // Settings
    data.insert("settings.title", "Mipangilio");
    data.insert("settings.language", "Lugha");
    data.insert("settings.language_desc", "Chagua lugha unayopendelea");
    data.insert("settings.stats", "Takwimu");
    data.insert("settings.version", "Toleo");
    data.insert("settings.storage", "Hifadhi");
    data.insert("settings.about", "Kuhusu");

    // Knowledge Graph & Footer
    data.insert("home.knowledge_graph", "Grafu ya Maarifa");
    data.insert("app.footer", "Maarifa kwa Wote");

    // === USB ===
    data.insert("usb.title", "Usawazishaji wa USB");
    data.insert("usb.scan", "Changanua");
    data.insert("usb.scanning", "Kutafuta vifaa vya USB");
    data.insert("usb.no_usb", "Hakuna Lazarus USB iliyogunduliwa");
    data.insert(
        "usb.no_usb_hint",
        "Ingiza USB yenye lazarus.sync au anzisha hapa chini",
    );
    data.insert("usb.error", "Imeshindwa kuchanganua");
    data.insert("usb.init_title", "Anzisha USB");
    data.insert(
        "usb.init_desc",
        "Unda Lazarus USB mpya kwa kushiriki nje ya mtandao",
    );
    data.insert("usb.init_btn", "Anzisha");
    data.insert("usb.init_error", "Imeshindwa kuanzisha");
    data.insert("usb.enter_path", "Ingiza njia ya USB");
    data.insert("usb.notes", "Vidokezo");
    data.insert("usb.posts", "Machapisho");
    data.insert("usb.packages", "Vifurushi");
    data.insert("usb.sync", "Sawazisha");
    data.insert("usb.export", "Hamisha nje");
    data.insert("usb.import", "Ingiza");
    data.insert("home.usb_sync", "Usawazishaji USB");
    data.insert("home.shortcut.wiki", "Wiki");
    data.insert("home.shortcut.usb", "USB");
    data.insert("home.shortcut.graph", "Grafu");
    data.insert("home.shortcut.review", "Kagua");
    Translations::new(data)
}
