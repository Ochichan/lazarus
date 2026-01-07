//! Bahasa Indonesia - Indonesian translations

use super::Translations;
use std::collections::HashMap;

pub fn translations() -> Translations {
    let mut data = HashMap::new();

    // Common
    data.insert("app.name", "Lazarus");
    data.insert("app.tagline", "Manajemen Pengetahuan Pribadi Offline");
    data.insert("nav.notes", "Catatan");
    data.insert("nav.search", "Cari");
    data.insert("nav.wiki", "Wiki");

    // Home
    data.insert("home.notes", "Catatan");
    data.insert("home.streak", "Hari Belajar");
    data.insert("home.day", "hari");
    data.insert("home.quick_start", "Mulai Cepat");
    data.insert("home.new_note", "Catatan Baru");
    data.insert("home.note_list", "Daftar Catatan");
    data.insert("home.split_view", "Tampilan Terpisah");
    data.insert("home.srs_review", "Ulasan SRS");
    data.insert("home.search", "Cari");
    data.insert("home.zim_manage", "Kelola ZIM");
    data.insert("home.security", "Keamanan");
    data.insert("home.shortcuts", "Pintasan Keyboard");
    data.insert("home.shortcut.new_note", "Catatan baru");
    data.insert("home.shortcut.search", "Cari");
    data.insert("home.shortcut.help", "Bantuan");

    // Editor
    data.insert("editor.title_placeholder", "Masukkan judul");
    data.insert("editor.content_placeholder", "Mulai menulis di sini...");
    data.insert("editor.tags", "Tag");
    data.insert("editor.tags_placeholder", "tag1, tag2, ...");
    data.insert("editor.edit_note", "Edit Catatan");
    data.insert("editor.encrypt", "Aktifkan enkripsi");
    data.insert("editor.focus", "Fokus");
    data.insert("editor.fullscreen", "Layar Penuh");
    data.insert("editor.save", "Simpan");
    data.insert("editor.saved", "Tersimpan");
    data.insert("editor.auto_saved", "Tersimpan otomatis");
    data.insert("editor.changed", "Berubah...");
    data.insert("editor.words", "kata");
    data.insert("editor.save_complete", "💾 Tersimpan!");
    data.insert("editor.encrypt_on", "🔒 Enkripsi aktif");
    data.insert("editor.encrypt_off", "🔓 Enkripsi nonaktif");
    data.insert(
        "editor.pin_required",
        "Silakan atur PIN terlebih dahulu (Menu Keamanan)",
    );
    data.insert("editor.pin_locked", "Silakan buka PIN terlebih dahulu");

    // Notes list
    data.insert("notes.title", "Catatan");
    data.insert("notes.export", "Ekspor");
    data.insert("notes.import", "Impor");
    data.insert("notes.no_notes", "Belum ada catatan");
    data.insert("notes.create_first", "Buat catatan pertama Anda!");
    data.insert("notes.no_title", "Tanpa Judul");
    data.insert("notes.edit", "Edit");
    data.insert("notes.delete", "Hapus");
    data.insert("notes.delete_confirm", "Yakin ingin menghapus?");
    data.insert("notes.encrypted", "Terenkripsi");
    data.insert("notes.created", "Dibuat");
    data.insert("notes.updated", "Diperbarui");
    data.insert("notes.find_duplicates", "Cari Duplikat");
    data.insert("notes.select_all", "Pilih Semua");
    data.insert("notes.selected", "dipilih");
    data.insert("notes.create_package", "Buat Paket");
    data.insert("notes.duplicates_title", "Catatan Duplikat");
    data.insert("notes.no_duplicates", "Tidak ada catatan duplikat");
    data.insert("notes.export_package", "Ekspor Paket");
    data.insert("notes.import_package", "Impor Paket");
    data.insert("notes.package_title", "Judul Paket");
    data.insert("notes.package_title_placeholder", "contoh: Fisika 101");
    data.insert("notes.package_author", "Penulis");
    data.insert("notes.package_author_placeholder", "Nama Anda");
    data.insert("notes.package_description", "Deskripsi");
    data.insert(
        "notes.package_description_placeholder",
        "Deskripsi paket (opsional)",
    );
    data.insert("notes.drop_file", "Letakkan file .laz di sini");
    data.insert("notes.or", "atau");
    data.insert("notes.select_file", "Pilih File");

    // Search
    data.insert("search.title", "Cari");
    data.insert("search.placeholder", "Masukkan kata pencarian...");
    data.insert("search.button", "Cari");
    data.insert("search.results", "Hasil Pencarian");
    data.insert("search.no_results", "Tidak ada hasil");
    data.insert("search.try_different", "Coba kata pencarian lain");
    data.insert("search.tips", "Tips Pencarian");
    data.insert(
        "search.tip1",
        "Beberapa kata mencari hasil yang mengandung semua kata",
    );
    data.insert("search.tip2", "Mencari di catatan dan Wikipedia");
    data.insert("search.tip3", "Kecocokan judul muncul pertama");

    // Wiki
    data.insert("wiki.search", "Cari Wiki");
    data.insert("wiki.recent_articles", "Recent Articles");
    data.insert("wiki.manage", "Kelola ZIM");
    data.insert("wiki.status", "Status");
    data.insert("wiki.loaded", "dimuat");
    data.insert("wiki.directory", "Direktori ZIM");
    data.insert(
        "wiki.directory_hint",
        "Salin file .zim ke folder ini dan segarkan",
    );
    data.insert("wiki.refresh", "Segarkan");
    data.insert("wiki.open_folder", "Buka Folder");
    data.insert("wiki.add", "Tambah ZIM");
    data.insert("wiki.add_placeholder", "Masukkan path file ZIM...");
    data.insert("wiki.loaded_files", "File ZIM Dimuat");
    data.insert("wiki.remove", "Hapus");
    data.insert("wiki.no_zim", "Tidak ada file ZIM");
    data.insert(
        "wiki.no_zim_hint",
        "Tambah file ZIM untuk menggunakan Wikipedia",
    );
    data.insert("wiki.no_zim_loaded", "Tidak ada file ZIM dimuat");
    data.insert("wiki.loaded_zims", "ZIM Dimuat");
    data.insert("wiki.add_btn", "Tambah");
    data.insert("wiki.add_hint", "Masukkan path lengkap file ZIM.");
    data.insert("wiki.name", "Nama");
    data.insert("wiki.path", "Path");
    data.insert("wiki.action", "Aksi");
    data.insert("wiki.zim_added", " ZIM ditambahkan: ");
    data.insert("wiki.no_new_zim", "Tidak ada file ZIM baru");
    data.insert("wiki.refresh_failed", "Gagal menyegarkan");
    data.insert("wiki.enter_path", "Silakan masukkan path");
    data.insert("wiki.add_failed", "Gagal menambah");
    data.insert("wiki.remove_confirm", "Hapus ZIM ini?");
    data.insert("wiki.zim_removed", "ZIM dihapus: ");
    data.insert("wiki.remove_failed", "Gagal menghapus");
    data.insert(
        "wiki.open_folder_msg",
        "Silakan buka folder di file explorer:",
    );

    // SRS
    data.insert("srs.title", "Ulasan SRS");
    data.insert("srs.cards", "Daftar Kartu");
    data.insert("srs.streak", "berturut-turut");
    data.insert("srs.show_answer", "Tampilkan Jawaban");
    data.insert("srs.again", "Lagi");
    data.insert("srs.hard", "Sulit");
    data.insert("srs.good", "Bagus");
    data.insert("srs.easy", "Mudah");
    data.insert("srs.complete", "🎉 Ulasan hari ini selesai!");
    data.insert("srs.no_cards", "Tidak ada kartu untuk diulas");
    data.insert("srs.progress", "kartu");
    data.insert("srs.today_review", "Ulasan Hari Ini");
    data.insert("srs.new_cards", "Baru");
    data.insert("srs.learning", "Belajar");
    data.insert("srs.mature", "Matang");
    data.insert("srs.create_from_notes", "Buat kartu dari catatan");
    data.insert("srs.create_cards", "Buat Kartu");
    data.insert("srs.cards_created", "kartu dibuat");
    data.insert("srs.no_cards_extracted", "Tidak ada kartu untuk diekstrak");
    data.insert("srs.repetitions", "Pengulangan");
    data.insert("srs.streak_days", "Berturut-turut");
    data.insert("srs.start_review", "Mulai Ulasan");
    data.insert("srs.no_cards_yet", "Belum ada kartu.");
    data.insert(
        "srs.create_hint",
        "Buka catatan dan klik tombol \"Buat Kartu\".",
    );
    data.insert("srs.interval", "Interval");
    data.insert("srs.days", " hari");
    data.insert("srs.delete_card_confirm", "Hapus kartu ini?");
    data.insert("srs.delete_failed", "Gagal menghapus");

    // Security
    data.insert("security.title", "Pengaturan Keamanan");
    data.insert("security.pin_not_set", "PIN belum diatur");
    data.insert("security.pin_enabled", "PIN aktif");
    data.insert("security.locked", "Terkunci - Masukkan PIN");
    data.insert("security.pin_input", "Masukkan PIN (6-32 karakter)");
    data.insert("security.set_pin", "Atur PIN");
    data.insert("security.remove_pin", "Hapus PIN");
    data.insert("security.lock", "Kunci");
    data.insert("security.unlock", "Buka");
    data.insert("security.wrong_pin", "PIN salah");
    data.insert("security.pin_set_success", "PIN telah diatur");
    data.insert("security.pin_removed", "PIN telah dihapus");
    data.insert("security.unlocked", "Terbuka");
    data.insert("security.pin_min_length", "PIN minimal 6 karakter");
    data.insert("security.enter_current_pin", "Masukkan PIN saat ini");

    // Backup
    data.insert("backup.title", "Cadangan");
    data.insert("backup.info", "Info Cadangan");
    data.insert("backup.now", "Cadangkan Sekarang");
    data.insert("backup.complete", "Cadangan selesai");
    data.insert(
        "backup.no_changes",
        "Tidak ada perubahan (cadangan dilewati)",
    );

    // Split view
    data.insert("split.select_note", "Pilih catatan dari kiri");

    // Common buttons
    data.insert("common.confirm", "Konfirmasi");
    data.insert("common.cancel", "Batal");
    data.insert("common.close", "Tutup");
    data.insert("common.loading", "Memuat...");
    data.insert("common.error", "Terjadi kesalahan");
    data.insert("common.success", "Berhasil");

    // Settings
    data.insert("settings.title", "Pengaturan");
    data.insert("settings.language", "Bahasa");
    data.insert("settings.language_desc", "Pilih bahasa yang Anda inginkan");
    data.insert("settings.stats", "Statistik");
    data.insert("settings.version", "Versi");
    data.insert("settings.storage", "Penyimpanan");
    data.insert("settings.about", "Tentang");

    // Knowledge Graph & Footer
    data.insert("home.knowledge_graph", "Grafik Pengetahuan");
    data.insert("app.footer", "Pengetahuan untuk Semua");

    // === USB ===
    data.insert("usb.title", "Sinkronisasi USB");
    data.insert("usb.scan", "Pindai");
    data.insert("usb.scanning", "Mencari perangkat USB");
    data.insert("usb.no_usb", "USB Lazarus tidak terdeteksi");
    data.insert(
        "usb.no_usb_hint",
        "Masukkan USB dengan lazarus.sync atau inisialisasi di bawah",
    );
    data.insert("usb.error", "Gagal memindai");
    data.insert("usb.init_title", "Inisialisasi USB");
    data.insert(
        "usb.init_desc",
        "Buat USB Lazarus baru untuk berbagi offline",
    );
    data.insert("usb.init_btn", "Inisialisasi");
    data.insert("usb.init_error", "Gagal inisialisasi");
    data.insert("usb.enter_path", "Masukkan path USB");
    data.insert("usb.notes", "Catatan");
    data.insert("usb.posts", "Postingan");
    data.insert("usb.packages", "Paket");
    data.insert("usb.sync", "Sinkronisasi");
    data.insert("usb.export", "Ekspor");
    data.insert("usb.import", "Impor");
    data.insert("home.usb_sync", "Sinkronisasi USB");
    data.insert("home.shortcut.wiki", "Wiki");
    data.insert("home.shortcut.usb", "USB");
    data.insert("home.shortcut.graph", "Grafik");
    data.insert("home.shortcut.review", "Review");
    Translations::new(data)
}
