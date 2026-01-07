//! 다국어 지원 (i18n)

use std::collections::HashMap;

mod ar;
mod bn;
mod en;
mod es;
mod fa;
mod fr;
mod hi;
mod id;
mod ja;
mod ko;
mod pt;
mod ru;
mod sw;
mod tr;
mod yue;
mod zh_cn;
mod zh_tw;

/// 지원 언어
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Lang {
    #[default]
    En,
    Ko,
    Ar,   // 아랍어
    Sw,   // 스와힐리어
    Id,   // 인도네시아어
    Hi,   // 힌디어
    Bn,   // 벵갈어
    Fa,   // 페르시아어 (RTL)
    Es,   // 스페인어
    Pt,   // 포르투갈어
    Fr,   // 불어
    Ru,   // 러시아어
    Tr,   // 터키어
    Ja,   // 일본어
    ZhCn, // 중국어 간체
    ZhTw, // 중국어 번체
    Yue,  // 광둥어
}

impl Lang {
    /// Accept-Language 헤더에서 언어 감지
    pub fn from_accept_language(header: Option<&str>) -> Self {
        let header = match header {
            Some(h) => h.to_lowercase(),
            None => return Self::default(),
        };

        if header.starts_with("ko") || header.contains("ko-") || header.contains("ko,") {
            Self::Ko
        } else if header.starts_with("ar") || header.contains("ar-") || header.contains("ar,") {
            Self::Ar
        } else if header.starts_with("sw") || header.contains("sw-") || header.contains("sw,") {
            Self::Sw
        } else if header.starts_with("id") || header.contains("id-") || header.contains("id,") {
            Self::Id
        } else if header.starts_with("hi") || header.contains("hi-") || header.contains("hi,") {
            Self::Hi
        } else if header.starts_with("bn") || header.contains("bn-") || header.contains("bn,") {
            Self::Bn
        } else if header.starts_with("es") || header.contains("es-") || header.contains("es,") {
            Self::Es
        } else if header.starts_with("pt") || header.contains("pt-") || header.contains("pt,") {
            Self::Pt
        } else if header.starts_with("fr") || header.contains("fr-") || header.contains("fr,") {
            Self::Fr
        } else if header.starts_with("ru") || header.contains("ru-") || header.contains("ru,") {
            Self::Ru
        } else if header.starts_with("tr") || header.contains("tr-") || header.contains("tr,") {
            Self::Tr
        } else if header.starts_with("ja") || header.contains("ja-") || header.contains("ja,") {
            Self::Ja
        } else if header.starts_with("yue") || header.contains("yue-") || header.contains("yue,") {
            Self::Yue
        } else if header.starts_with("zh-tw") || header.starts_with("zh-hant") {
            Self::ZhTw
        } else if header.starts_with("zh") || header.contains("zh-") || header.contains("zh,") {
            Self::ZhCn
        } else {
            Self::En
        }
    }

    /// 언어 코드
    pub fn code(&self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Ko => "ko",
            Self::Ar => "ar",
            Self::Sw => "sw",
            Self::Id => "id",
            Self::Hi => "hi",
            Self::Bn => "bn",
            Self::Fa => "fa",
            Self::Es => "es",
            Self::Pt => "pt",
            Self::Fr => "fr",
            Self::Ru => "ru",
            Self::Tr => "tr",
            Self::Ja => "ja",
            Self::ZhCn => "zh-CN",
            Self::ZhTw => "zh-TW",
            Self::Yue => "yue",
        }
    }

    /// RTL (오른쪽→왼쪽) 언어인지
    pub fn is_rtl(&self) -> bool {
        matches!(self, Self::Ar | Self::Fa)
    }

    /// 언어 이름 (해당 언어로)
    pub fn native_name(&self) -> &'static str {
        match self {
            Self::En => "English",
            Self::Ko => "한국어",
            Self::Ar => "العربية",
            Self::Sw => "Kiswahili",
            Self::Id => "Bahasa Indonesia",
            Self::Hi => "हिन्दी",
            Self::Bn => "বাংলা",
            Self::Fa => "فارسی",
            Self::Es => "Español",
            Self::Pt => "Português",
            Self::Fr => "Français",
            Self::Ru => "Русский",
            Self::Tr => "Türkçe",
            Self::Ja => "日本語",
            Self::ZhCn => "简体中文",
            Self::ZhTw => "繁體中文",
            Self::Yue => "廣東話",
        }
    }

    /// 모든 언어 목록
    pub fn all() -> &'static [Lang] {
        &[
            Self::En,
            Self::Ko,
            Self::Ar,
            Self::Sw,
            Self::Id,
            Self::Hi,
            Self::Bn,
            Self::Fa,
            Self::Es,
            Self::Pt,
            Self::Fr,
            Self::Ru,
            Self::Tr,
            Self::Ja,
            Self::ZhCn,
            Self::ZhTw,
            Self::Yue,
        ]
    }
}

/// 번역 키
pub struct Translations {
    data: HashMap<&'static str, &'static str>,
}

impl Translations {
    pub fn new(data: HashMap<&'static str, &'static str>) -> Self {
        Self { data }
    }

    pub fn get<'a>(&'a self, key: &'a str) -> &'a str {
        self.data.get(key).copied().unwrap_or(key)
    }
}

/// 언어에 맞는 번역 가져오기
pub fn get_translations(lang: Lang) -> Translations {
    match lang {
        Lang::En => en::translations(),
        Lang::Ko => ko::translations(),
        Lang::Ar => ar::translations(),
        Lang::Sw => sw::translations(),
        Lang::Id => id::translations(),
        Lang::Hi => hi::translations(),
        Lang::Bn => bn::translations(),
        Lang::Fa => fa::translations(),
        Lang::Es => es::translations(),
        Lang::Pt => pt::translations(),
        Lang::Fr => fr::translations(),
        Lang::Ru => ru::translations(),
        Lang::Tr => tr::translations(),
        Lang::Ja => ja::translations(),
        Lang::ZhCn => zh_cn::translations(),
        Lang::ZhTw => zh_tw::translations(),
        Lang::Yue => yue::translations(),
    }
}

/// 모든 번역을 HashMap<String, String>으로 반환
pub fn all_translations(lang: Lang) -> HashMap<String, String> {
    let tr = get_translations(lang);
    let keys = translation_keys();

    let mut map = HashMap::new();
    for key in keys {
        map.insert(key.to_string(), tr.get(key).to_string());
    }
    map
}

/// 모든 번역 키 목록
fn translation_keys() -> &'static [&'static str] {
    &[
        // 공통
        "app.name",
        "app.tagline",
        "nav.notes",
        "nav.search",
        "nav.wiki",
        // 메인 화면
        "home.notes",
        "home.streak",
        "home.day",
        "home.quick_start",
        "home.new_note",
        "home.note_list",
        "home.split_view",
        "home.srs_review",
        "home.search",
        "home.zim_manage",
        "home.security",
        "home.shortcuts",
        "home.shortcut.new_note",
        "home.shortcut.search",
        "home.shortcut.help",
        "home.knowledge_graph",
        "home.usb_sync",
        "home.shortcut.wiki",
        "home.shortcut.usb",
        "home.shortcut.graph",
        "home.shortcut.review",
        "app.footer",
        // 에디터
        "editor.title_placeholder",
        "editor.content_placeholder",
        "editor.tags",
        "editor.tags_placeholder",
        "editor.edit_note",
        "editor.encrypt",
        "editor.focus",
        "editor.fullscreen",
        "editor.save",
        "editor.saved",
        "editor.auto_saved",
        "editor.changed",
        "editor.words",
        "editor.save_complete",
        "editor.encrypt_on",
        "editor.encrypt_off",
        "editor.pin_required",
        "editor.pin_locked",
        // 노트 목록
        "notes.title",
        "notes.export",
        "notes.import",
        "notes.no_notes",
        "notes.create_first",
        "notes.no_title",
        "notes.edit",
        "notes.delete",
        "notes.delete_confirm",
        "notes.encrypted",
        "notes.created",
        "notes.updated",
        "notes.find_duplicates",
        "notes.select_all",
        "notes.selected",
        "notes.create_package",
        "notes.duplicates_title",
        "notes.no_duplicates",
        "notes.export_package",
        "notes.import_package",
        "notes.package_title",
        "notes.package_title_placeholder",
        "notes.package_author",
        "notes.package_author_placeholder",
        "notes.package_description",
        "notes.package_description_placeholder",
        "notes.drop_file",
        "notes.or",
        "notes.select_file",
        // 검색
        "search.title",
        "search.placeholder",
        "search.button",
        "search.results",
        "search.no_results",
        "search.try_different",
        "search.tips",
        "search.tip1",
        "search.tip2",
        "search.tip3",
        // 위키
        "wiki.search",
        "wiki.recent_articles",
        "wiki.manage",
        "wiki.status",
        "wiki.loaded",
        "wiki.directory",
        "wiki.directory_hint",
        "wiki.refresh",
        "wiki.open_folder",
        "wiki.add",
        "wiki.add_placeholder",
        "wiki.loaded_files",
        "wiki.remove",
        "wiki.no_zim",
        "wiki.no_zim_hint",
        "wiki.no_zim_loaded",
        "wiki.loaded_zims",
        "wiki.add_btn",
        "wiki.add_hint",
        "wiki.name",
        "wiki.path",
        "wiki.action",
        "wiki.zim_added",
        "wiki.no_new_zim",
        "wiki.refresh_failed",
        "wiki.enter_path",
        "wiki.add_failed",
        "wiki.remove_confirm",
        "wiki.zim_removed",
        "wiki.remove_failed",
        "wiki.open_folder_msg",
        // SRS
        "srs.title",
        "srs.cards",
        "srs.streak",
        "srs.show_answer",
        "srs.again",
        "srs.hard",
        "srs.good",
        "srs.easy",
        "srs.complete",
        "srs.no_cards",
        "srs.progress",
        "srs.today_review",
        "srs.new_cards",
        "srs.learning",
        "srs.mature",
        "srs.create_from_notes",
        "srs.create_cards",
        "srs.cards_created",
        "srs.no_cards_extracted",
        "srs.repetitions",
        "srs.streak_days",
        "srs.start_review",
        "srs.no_cards_yet",
        "srs.create_hint",
        "srs.interval",
        "srs.days",
        "srs.delete_card_confirm",
        "srs.delete_failed",
        // 보안
        "security.title",
        "security.pin_not_set",
        "security.pin_enabled",
        "security.locked",
        "security.pin_input",
        "security.set_pin",
        "security.remove_pin",
        "security.lock",
        "security.unlock",
        "security.wrong_pin",
        "security.pin_set_success",
        "security.pin_removed",
        "security.unlocked",
        "security.pin_min_length",
        "security.enter_current_pin",
        // 백업
        "backup.title",
        "backup.info",
        "backup.now",
        "backup.complete",
        "backup.no_changes",
        // 스플릿뷰
        "split.select_note",
        // 공통 버튼
        "common.confirm",
        "common.cancel",
        "common.close",
        "common.loading",
        "common.error",
        "common.success",
        // Settings
        "settings.title",
        "settings.language",
        "settings.language_desc",
        "settings.stats",
        "settings.version",
        "settings.storage",
        "settings.about",
        // USB
        "usb.title",
        "usb.scan",
        "usb.scanning",
        "usb.no_usb",
        "usb.no_usb_hint",
        "usb.error",
        "usb.init_title",
        "usb.init_desc",
        "usb.init_btn",
        "usb.init_error",
        "usb.enter_path",
        "usb.notes",
        "usb.posts",
        "usb.packages",
        "usb.sync",
        "usb.export",
        "usb.import",
    ]
}
