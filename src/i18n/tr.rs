//! Turkish (Türkçe) translations

use std::collections::HashMap;
use super::Translations;

pub fn translations() -> Translations {
    let mut data = HashMap::new();

    // Common
    data.insert("app.name", "Lazarus");
    data.insert("app.tagline", "Çevrimdışı Kişisel Bilgi Yönetimi");
    data.insert("nav.notes", "Notlar");
    data.insert("nav.search", "Ara");
    data.insert("nav.wiki", "Wiki");

    // Home
    data.insert("home.notes", "Notlar");
    data.insert("home.streak", "Çalışma Serisi");
    data.insert("home.day", "gün");
    data.insert("home.quick_start", "Hızlı Başlangıç");
    data.insert("home.new_note", "Yeni Not");
    data.insert("home.note_list", "Not Listesi");
    data.insert("home.split_view", "Bölünmüş Görünüm");
    data.insert("home.srs_review", "SRS Tekrar");
    data.insert("home.search", "Ara");
    data.insert("home.zim_manage", "ZIM Yönetimi");
    data.insert("home.security", "Güvenlik");
    data.insert("home.shortcuts", "Klavye Kısayolları");
    data.insert("home.shortcut.new_note", "Yeni not");
    data.insert("home.shortcut.search", "Ara");
    data.insert("home.shortcut.help", "Yardım");

    // Editor
    data.insert("editor.title_placeholder", "Başlık girin");
    data.insert("editor.content_placeholder", "Yazmaya başlayın...");
    data.insert("editor.tags", "Etiketler");
    data.insert("editor.tags_placeholder", "etiket1, etiket2, ...");
    data.insert("editor.edit_note", "Notu Düzenle");
    data.insert("editor.encrypt", "Şifrelemeyi aç/kapat");
    data.insert("editor.focus", "Odaklan");
    data.insert("editor.fullscreen", "Tam Ekran");
    data.insert("editor.save", "Kaydet");
    data.insert("editor.saved", "Kaydedildi");
    data.insert("editor.auto_saved", "Otomatik kaydedildi");
    data.insert("editor.changed", "Değişti...");
    data.insert("editor.words", "kelime");
    data.insert("editor.save_complete", "💾 Kaydedildi!");
    data.insert("editor.encrypt_on", "🔒 Şifreleme etkin");
    data.insert("editor.encrypt_off", "🔓 Şifreleme devre dışı");
    data.insert("editor.pin_required", "Önce PIN ayarlayın (Güvenlik menüsü)");
    data.insert("editor.pin_locked", "Önce PIN ile kilidi açın");

    // Notes list
    data.insert("notes.title", "Notlar");
    data.insert("notes.export", "Dışa Aktar");
    data.insert("notes.import", "İçe Aktar");
    data.insert("notes.no_notes", "Henüz not yok");
    data.insert("notes.create_first", "İlk notunuzu oluşturun!");
    data.insert("notes.no_title", "Başlıksız");
    data.insert("notes.edit", "Düzenle");
    data.insert("notes.delete", "Sil");
    data.insert("notes.delete_confirm", "Silmek istediğinizden emin misiniz?");
    data.insert("notes.encrypted", "Şifreli");
    data.insert("notes.created", "Oluşturuldu");
    data.insert("notes.updated", "Güncellendi");
    data.insert("notes.find_duplicates", "Kopyaları Bul");
    data.insert("notes.select_all", "Tümünü Seç");
    data.insert("notes.selected", "seçildi");
    data.insert("notes.create_package", "Paket Oluştur");
    data.insert("notes.duplicates_title", "Kopya Notlar");
    data.insert("notes.no_duplicates", "Kopya not bulunamadı");
    data.insert("notes.export_package", "Paketi Dışa Aktar");
    data.insert("notes.import_package", "Paketi İçe Aktar");
    data.insert("notes.package_title", "Paket Başlığı");
    data.insert("notes.package_title_placeholder", "örn. Fizik 101");
    data.insert("notes.package_author", "Yazar");
    data.insert("notes.package_author_placeholder", "Adınız");
    data.insert("notes.package_description", "Açıklama");
    data.insert("notes.package_description_placeholder", "Paket açıklaması (isteğe bağlı)");
    data.insert("notes.drop_file", ".laz dosyasını buraya bırakın");
    data.insert("notes.or", "veya");
    data.insert("notes.select_file", "Dosya Seç");

    // Search
    data.insert("search.title", "Ara");
    data.insert("search.placeholder", "Arama terimi girin...");
    data.insert("search.button", "Ara");
    data.insert("search.results", "Arama Sonuçları");
    data.insert("search.no_results", "Sonuç bulunamadı");
    data.insert("search.try_different", "Farklı bir terim deneyin");
    data.insert("search.tips", "Arama İpuçları");
    data.insert("search.tip1", "Birden fazla kelime tüm terimleri içeren sonuçları arar");
    data.insert("search.tip2", "Hem notlarda hem Vikipedi'de arar");
    data.insert("search.tip3", "Başlık eşleşmeleri önce gösterilir");

    // Wiki
    data.insert("wiki.search", "Wiki Ara");
    data.insert("wiki.manage", "ZIM Yönetimi");
    data.insert("wiki.status", "Durum");
    data.insert("wiki.loaded", "yüklendi");
    data.insert("wiki.directory", "ZIM Dizini");
    data.insert("wiki.directory_hint", "Bu klasöre .zim dosyalarını kopyalayın ve yenileyin");
    data.insert("wiki.refresh", "Yenile");
    data.insert("wiki.open_folder", "Klasörü Aç");
    data.insert("wiki.add", "ZIM Ekle");
    data.insert("wiki.add_placeholder", "ZIM dosya yolunu girin...");
    data.insert("wiki.loaded_files", "Yüklenen ZIM Dosyaları");
    data.insert("wiki.remove", "Kaldır");
    data.insert("wiki.no_zim", "ZIM dosyası yok");
    data.insert("wiki.no_zim_hint", "Vikipedi kullanmak için ZIM dosyası ekleyin");
    data.insert("wiki.no_zim_loaded", "ZIM dosyası yüklenmedi");
    data.insert("wiki.loaded_zims", "Yüklenen ZIM'ler");
    data.insert("wiki.add_btn", "Ekle");
    data.insert("wiki.add_hint", "ZIM dosyasının tam yolunu girin.");
    data.insert("wiki.name", "Ad");
    data.insert("wiki.path", "Yol");
    data.insert("wiki.action", "İşlem");
    data.insert("wiki.zim_added", " ZIM eklendi: ");
    data.insert("wiki.no_new_zim", "Yeni ZIM dosyası yok");
    data.insert("wiki.refresh_failed", "Yenileme başarısız");
    data.insert("wiki.enter_path", "Yol girin");
    data.insert("wiki.add_failed", "Ekleme başarısız");
    data.insert("wiki.remove_confirm", "Bu ZIM'i kaldırmak istiyor musunuz?");
    data.insert("wiki.zim_removed", "ZIM kaldırıldı: ");
    data.insert("wiki.remove_failed", "Kaldırma başarısız");
    data.insert("wiki.open_folder_msg", "Dosya gezgininde klasörü açın:");

    // SRS
    data.insert("srs.title", "SRS Tekrar");
    data.insert("srs.cards", "Kart Listesi");
    data.insert("srs.streak", "seri");
    data.insert("srs.show_answer", "Cevabı Göster");
    data.insert("srs.again", "Tekrar");
    data.insert("srs.hard", "Zor");
    data.insert("srs.good", "İyi");
    data.insert("srs.easy", "Kolay");
    data.insert("srs.complete", "🎉 Bugünkü tekrar tamamlandı!");
    data.insert("srs.no_cards", "Tekrar edilecek kart yok");
    data.insert("srs.progress", "kart");
    data.insert("srs.today_review", "Bugün Kalan");
    data.insert("srs.new_cards", "Yeni");
    data.insert("srs.learning", "Öğreniliyor");
    data.insert("srs.mature", "Olgun");
    data.insert("srs.create_from_notes", "Notlardan kart oluştur");
    data.insert("srs.create_cards", "Kart Oluştur");
    data.insert("srs.cards_created", "kart oluşturuldu");
    data.insert("srs.no_cards_extracted", "Çıkarılacak kart yok");
    data.insert("srs.repetitions", "Tekrar");
    data.insert("srs.streak_days", "Seri");
    data.insert("srs.start_review", "Tekrara Başla");
    data.insert("srs.no_cards_yet", "Henüz kart yok.");
    data.insert("srs.create_hint", "Bir not açın ve \"Kart Oluştur\" düğmesine tıklayın.");
    data.insert("srs.interval", "Aralık");
    data.insert("srs.days", " gün");
    data.insert("srs.delete_card_confirm", "Bu kartı silmek istiyor musunuz?");
    data.insert("srs.delete_failed", "Silme başarısız");

    // Security
    data.insert("security.title", "Güvenlik Ayarları");
    data.insert("security.pin_not_set", "PIN ayarlanmadı");
    data.insert("security.pin_enabled", "PIN etkin");
    data.insert("security.locked", "Kilitli - PIN girin");
    data.insert("security.pin_input", "PIN girin (6-32 karakter)");
    data.insert("security.set_pin", "PIN Ayarla");
    data.insert("security.remove_pin", "PIN Kaldır");
    data.insert("security.lock", "Kilitle");
    data.insert("security.unlock", "Kilidi Aç");
    data.insert("security.wrong_pin", "Yanlış PIN");
    data.insert("security.pin_set_success", "PIN ayarlandı");
    data.insert("security.pin_removed", "PIN kaldırıldı");
    data.insert("security.unlocked", "Kilit açıldı");
    data.insert("security.pin_min_length", "PIN en az 6 karakter olmalı");
    data.insert("security.enter_current_pin", "Mevcut PIN'i girin");

    // Backup
    data.insert("backup.title", "Yedekleme");
    data.insert("backup.info", "Yedek Bilgisi");
    data.insert("backup.now", "Şimdi Yedekle");
    data.insert("backup.complete", "Yedekleme tamamlandı");
    data.insert("backup.no_changes", "Değişiklik yok (yedekleme atlandı)");

    // Split view
    data.insert("split.select_note", "Soldan bir not seçin");

    // Common buttons
    data.insert("common.confirm", "Onayla");
    data.insert("common.cancel", "İptal");
    data.insert("common.close", "Kapat");
    data.insert("common.loading", "Yükleniyor...");
    data.insert("common.error", "Bir hata oluştu");
    data.insert("common.success", "Başarılı");

    // Settings
    data.insert("settings.title", "Ayarlar");
    data.insert("settings.language", "Dil");
    data.insert("settings.language_desc", "Tercih ettiğiniz dili seçin");
    data.insert("settings.stats", "İstatistikler");
    data.insert("settings.version", "Sürüm");
    data.insert("settings.storage", "Depolama");
    data.insert("settings.about", "Hakkında");

    // Knowledge Graph & Footer
    data.insert("home.knowledge_graph", "Bilgi Grafiği");
    data.insert("app.footer", "Herkes İçin Bilgi");

    Translations::new(data)
}
