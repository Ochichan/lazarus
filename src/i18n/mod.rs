//! 다국어 지원 (i18n)

use std::collections::HashMap;

/// 지원 언어
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Lang {
    #[default]
    Ko,
    En,
}

impl Lang {
    /// Accept-Language 헤더에서 언어 감지
    pub fn from_accept_language(header: Option<&str>) -> Self {
        let header = match header {
            Some(h) => h.to_lowercase(),
            None => return Self::default(),
        };

        if header.starts_with("en") || header.contains("en-") || header.contains("en,") {
            Self::En
        } else {
            Self::Ko
        }
    }

    /// 언어 코드
    pub fn code(&self) -> &'static str {
        match self {
            Self::Ko => "ko",
            Self::En => "en",
        }
    }
}

/// 번역 키
pub struct Translations {
    data: HashMap<&'static str, &'static str>,
}

impl Translations {
    pub fn get<'a>(&'a self, key: &'a str) -> &'a str {
        self.data.get(key).copied().unwrap_or(key)
    }
}

/// 한국어 번역
pub fn ko() -> Translations {
    let mut data = HashMap::new();
    data.insert("notes.no_title", "제목 없음");
    // 공통
    data.insert("app.name", "Lazarus");
    data.insert("app.tagline", "오프라인에서도 작동하는 개인 지식 관리");
    data.insert("nav.notes", "노트");
    data.insert("nav.search", "검색");
    data.insert("nav.wiki", "위키");

    data.insert("notes.find_duplicates", "중복 찾기");
    data.insert("notes.select_all", "전체 선택");
    data.insert("notes.selected", "개 선택됨");
    data.insert("notes.create_package", "패키지 생성");
    data.insert("notes.duplicates_title", "중복 노트 관리");
    data.insert("notes.no_duplicates", "중복 노트가 없습니다");
    data.insert("notes.export_package", "패키지 내보내기");
    data.insert("notes.import_package", "패키지 가져오기");
    data.insert("notes.package_title", "패키지 제목");
    data.insert("notes.package_title_placeholder", "예: 물리학 101");
    data.insert("notes.package_author", "작성자");
    data.insert("notes.package_author_placeholder", "이름");
    data.insert("notes.package_description", "설명");
    data.insert("notes.package_description_placeholder", "패키지 설명 (선택)");
    data.insert("notes.drop_file", ".laz 파일을 여기에 드롭하세요");
    data.insert("notes.or", "또는");
    data.insert("notes.select_file", "파일 선택");
    data.insert("editor.edit_note", "노트 수정");
    data.insert("notes.created", "생성");
    data.insert("notes.updated", "수정");
    data.insert("srs.create_cards", "카드 생성");
    data.insert("srs.cards_created", "개 카드가 생성되었습니다");
    data.insert("srs.no_cards_extracted", "추출할 카드가 없습니다");
    data.insert("search.tips", "검색 팁");
    data.insert("search.tip1", "여러 단어를 입력하면 모두 포함된 결과를 찾습니다");
    data.insert("search.tip2", "노트와 위키백과를 동시에 검색합니다");
    data.insert("search.tip3", "제목에 일치하는 결과가 먼저 표시됩니다");
    data.insert("split.select_note", "왼쪽에서 노트를 선택하세요");
    data.insert("srs.today_review", "오늘 복습");
    data.insert("srs.new_cards", "새 카드");
    data.insert("srs.learning", "학습 중");
    data.insert("srs.mature", "완료");
    data.insert("srs.create_from_notes", "노트에서 카드 만들기");
    data.insert("srs.repetitions", "반복");
    data.insert("srs.streak_days", "연속");
    data.insert("srs.start_review", "복습하기");
    data.insert("srs.no_cards_yet", "아직 카드가 없습니다.");
    data.insert("srs.create_hint", "노트를 열고 \"카드 생성\" 버튼을 클릭하세요.");
    data.insert("srs.interval", "간격");
    data.insert("srs.days", "일");
    data.insert("srs.delete_card_confirm", "이 카드를 삭제하시겠습니까?");
    data.insert("srs.delete_failed", "삭제 실패");
    data.insert("security.enter_current_pin", "현재 PIN을 입력하세요");
    data.insert("wiki.no_zim_loaded", "로드된 ZIM 파일이 없습니다");
    data.insert("wiki.loaded_zims", "로드된 ZIM");
    data.insert("wiki.add_btn", "추가");
    data.insert("wiki.add_hint", "ZIM 파일의 전체 경로를 입력하세요.");
    data.insert("wiki.name", "이름");
    data.insert("wiki.path", "경로");
    data.insert("wiki.action", "작업");
    data.insert("wiki.zim_added", "개의 ZIM 추가됨: ");
    data.insert("wiki.no_new_zim", "새로운 ZIM 파일 없음");
    data.insert("wiki.refresh_failed", "새로고침 실패");
    data.insert("wiki.enter_path", "경로를 입력하세요");
    data.insert("wiki.add_failed", "추가 실패");
    data.insert("wiki.remove_confirm", "ZIM을 제거하시겠습니까?");
    data.insert("wiki.zim_removed", "ZIM 제거됨: ");
    data.insert("wiki.remove_failed", "제거 실패");
    data.insert("wiki.open_folder_msg", "파일 탐색기에서 직접 폴더를 열어주세요:");
    
    // 메인 화면
    data.insert("home.notes", "노트");
    data.insert("home.streak", "연속 학습");
    data.insert("home.day", "일");
    data.insert("home.quick_start", "빠른 시작");
    data.insert("home.new_note", "새 노트");
    data.insert("home.note_list", "노트 목록");
    data.insert("home.split_view", "스플릿뷰");
    data.insert("home.srs_review", "SRS 복습");
    data.insert("home.search", "검색");
    data.insert("home.zim_manage", "ZIM 관리");
    data.insert("home.security", "보안");
    data.insert("home.shortcuts", "키보드 단축키");
    data.insert("home.shortcut.new_note", "새 노트");
    data.insert("home.shortcut.search", "검색");
    data.insert("home.shortcut.help", "도움말");
    
    // 에디터
    data.insert("editor.title_placeholder", "제목을 입력하세요");
    data.insert("editor.content_placeholder", "여기에 내용을 입력하세요...");
    data.insert("editor.tags", "태그");
    data.insert("editor.tags_placeholder", "태그1, 태그2, ...");
    data.insert("editor.encrypt", "암호화 토글");
    data.insert("editor.focus", "집중");
    data.insert("editor.fullscreen", "전체화면");
    data.insert("editor.save", "저장");
    data.insert("editor.saved", "저장됨");
    data.insert("editor.auto_saved", "자동 저장됨");
    data.insert("editor.changed", "변경됨...");
    data.insert("editor.words", "단어");
    data.insert("editor.save_complete", "💾 저장 완료!");
    data.insert("editor.encrypt_on", "🔒 암호화 활성화");
    data.insert("editor.encrypt_off", "🔓 암호화 해제");
    data.insert("editor.pin_required", "먼저 PIN을 설정하세요 (보안 메뉴)");
    data.insert("editor.pin_locked", "PIN 잠금을 해제하세요");
    
    // 노트 목록
    data.insert("notes.title", "노트 목록");
    data.insert("notes.export", "내보내기");
    data.insert("notes.import", "가져오기");
    data.insert("notes.no_notes", "노트가 없습니다");
    data.insert("notes.create_first", "첫 번째 노트를 만들어보세요!");
    data.insert("notes.edit", "편집");
    data.insert("notes.delete", "삭제");
    data.insert("notes.delete_confirm", "정말 삭제하시겠습니까?");
    data.insert("notes.encrypted", "암호화됨");
    
    // 검색
    data.insert("search.title", "검색");
    data.insert("search.placeholder", "검색어 입력...");
    data.insert("search.button", "검색");
    data.insert("search.results", "검색 결과");
    data.insert("search.no_results", "검색 결과가 없습니다");
    data.insert("search.try_different", "다른 검색어를 시도해보세요");
    
    // 위키
    data.insert("wiki.search", "위키 검색");
    data.insert("wiki.manage", "ZIM 관리");
    data.insert("wiki.status", "현황");
    data.insert("wiki.loaded", "개 로드됨");
    data.insert("wiki.directory", "ZIM 디렉토리");
    data.insert("wiki.directory_hint", "이 폴더에 .zim 파일을 복사 후 새로고침");
    data.insert("wiki.refresh", "새로고침");
    data.insert("wiki.open_folder", "폴더 열기");
    data.insert("wiki.add", "ZIM 추가");
    data.insert("wiki.add_placeholder", "ZIM 파일 경로 입력...");
    data.insert("wiki.loaded_files", "로드된 ZIM 파일");
    data.insert("wiki.remove", "제거");
    data.insert("wiki.no_zim", "ZIM 파일 없음");
    data.insert("wiki.no_zim_hint", "위키백과를 사용하려면 ZIM 파일을 추가하세요");
    
    // SRS
    data.insert("srs.title", "SRS 복습");
    data.insert("srs.cards", "카드 목록");
    data.insert("srs.streak", "연속");
    data.insert("srs.show_answer", "정답 보기");
    data.insert("srs.again", "다시");
    data.insert("srs.hard", "어려움");
    data.insert("srs.good", "좋음");
    data.insert("srs.easy", "쉬움");
    data.insert("srs.complete", "🎉 오늘의 복습 완료!");
    data.insert("srs.no_cards", "복습할 카드가 없습니다");
    data.insert("srs.progress", "카드");
    
    // 보안
    data.insert("security.title", "보안 설정");
    data.insert("security.pin_not_set", "PIN이 설정되지 않았습니다");
    data.insert("security.pin_enabled", "PIN 활성화됨");
    data.insert("security.locked", "잠금됨 - PIN을 입력하세요");
    data.insert("security.pin_input", "PIN 입력 (6-32자리)");
    data.insert("security.set_pin", "PIN 설정");
    data.insert("security.remove_pin", "PIN 제거");
    data.insert("security.lock", "잠금");
    data.insert("security.unlock", "잠금 해제");
    data.insert("security.wrong_pin", "잘못된 PIN입니다");
    data.insert("security.pin_set_success", "PIN이 설정되었습니다");
    data.insert("security.pin_removed", "PIN이 제거되었습니다");
    data.insert("security.unlocked", "잠금이 해제되었습니다");
    data.insert("security.pin_min_length", "PIN을 6자리 이상 입력하세요");
    
    // 백업
    data.insert("backup.title", "백업");
    data.insert("backup.info", "백업 정보");
    data.insert("backup.now", "지금 백업");
    data.insert("backup.complete", "백업 완료");
    data.insert("backup.no_changes", "변경사항 없음 (백업 스킵)");
    
    // 공통 버튼/메시지
    data.insert("common.confirm", "확인");
    data.insert("common.cancel", "취소");
    data.insert("common.close", "닫기");
    data.insert("common.loading", "로딩 중...");
    data.insert("common.error", "오류가 발생했습니다");
    data.insert("common.success", "성공");
    
    Translations { data }
}

/// 영어 번역
pub fn en() -> Translations {
    let mut data = HashMap::new();
    data.insert("notes.no_title", "Untitled");
    // Common
    data.insert("app.name", "Lazarus");
    data.insert("app.tagline", "Offline Personal Knowledge Management");
    data.insert("nav.notes", "Notes");
    data.insert("nav.search", "Search");
    data.insert("nav.wiki", "Wiki");

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
    data.insert("notes.package_description_placeholder", "Package description (optional)");
    data.insert("notes.drop_file", "Drop .laz file here");
    data.insert("notes.or", "or");
    data.insert("notes.select_file", "Select File");
    data.insert("editor.edit_note", "Edit Note");
    data.insert("notes.created", "Created");
    data.insert("notes.updated", "Updated");
    data.insert("srs.create_cards", "Create Cards");
    data.insert("srs.cards_created", "cards created");
    data.insert("srs.no_cards_extracted", "No cards to extract");
    data.insert("search.tips", "Search Tips");
    data.insert("search.tip1", "Multiple words search for results containing all terms");
    data.insert("search.tip2", "Searches both notes and Wikipedia");
    data.insert("search.tip3", "Title matches appear first");
    data.insert("split.select_note", "Select a note from the left");
    data.insert("srs.today_review", "Due Today");
    data.insert("srs.new_cards", "New");
    data.insert("srs.learning", "Learning");
    data.insert("srs.mature", "Mature");
    data.insert("srs.create_from_notes", "Create cards from notes");
    data.insert("srs.repetitions", "Reps");
    data.insert("srs.streak_days", "Streak");
    data.insert("srs.start_review", "Start Review");
    data.insert("srs.no_cards_yet", "No cards yet.");
    data.insert("srs.create_hint", "Open a note and click \"Create Cards\" button.");
    data.insert("srs.interval", "Interval");
    data.insert("srs.days", " days");
    data.insert("srs.delete_card_confirm", "Delete this card?");
    data.insert("srs.delete_failed", "Delete failed");
    data.insert("security.enter_current_pin", "Enter current PIN");
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
    data.insert("wiki.open_folder_msg", "Please open the folder in file explorer:");
    
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
    data.insert("editor.pin_required", "Please set PIN first (Security menu)");
    data.insert("editor.pin_locked", "Please unlock PIN first");
    
    // Notes list
    data.insert("notes.title", "Notes");
    data.insert("notes.export", "Export");
    data.insert("notes.import", "Import");
    data.insert("notes.no_notes", "No notes yet");
    data.insert("notes.create_first", "Create your first note!");
    data.insert("notes.edit", "Edit");
    data.insert("notes.delete", "Delete");
    data.insert("notes.delete_confirm", "Are you sure you want to delete?");
    data.insert("notes.encrypted", "Encrypted");
    
    // Search
    data.insert("search.title", "Search");
    data.insert("search.placeholder", "Enter search term...");
    data.insert("search.button", "Search");
    data.insert("search.results", "Search Results");
    data.insert("search.no_results", "No results found");
    data.insert("search.try_different", "Try a different search term");
    
    // Wiki
    data.insert("wiki.search", "Wiki Search");
    data.insert("wiki.manage", "ZIM Management");
    data.insert("wiki.status", "Status");
    data.insert("wiki.loaded", "loaded");
    data.insert("wiki.directory", "ZIM Directory");
    data.insert("wiki.directory_hint", "Copy .zim files to this folder and refresh");
    data.insert("wiki.refresh", "Refresh");
    data.insert("wiki.open_folder", "Open Folder");
    data.insert("wiki.add", "Add ZIM");
    data.insert("wiki.add_placeholder", "Enter ZIM file path...");
    data.insert("wiki.loaded_files", "Loaded ZIM Files");
    data.insert("wiki.remove", "Remove");
    data.insert("wiki.no_zim", "No ZIM files");
    data.insert("wiki.no_zim_hint", "Add a ZIM file to use Wikipedia");
    
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
    data.insert("security.pin_min_length", "PIN must be at least 6 characters");
    
    // Backup
    data.insert("backup.title", "Backup");
    data.insert("backup.info", "Backup Info");
    data.insert("backup.now", "Backup Now");
    data.insert("backup.complete", "Backup complete");
    data.insert("backup.no_changes", "No changes (backup skipped)");
    
    // Common buttons/messages
    data.insert("common.confirm", "Confirm");
    data.insert("common.cancel", "Cancel");
    data.insert("common.close", "Close");
    data.insert("common.loading", "Loading...");
    data.insert("common.error", "An error occurred");
    data.insert("common.success", "Success");
    
    Translations { data }
}

/// 언어에 맞는 번역 가져오기
pub fn get_translations(lang: Lang) -> Translations {
    match lang {
        Lang::Ko => ko(),
        Lang::En => en(),
    }
}

/// 모든 번역을 HashMap<String, String>으로 반환
pub fn all_translations(lang: Lang) -> HashMap<String, String> {
    let tr = get_translations(lang);
    let keys = [
    	"notes.title", "notes.export", "notes.import", "split.select_note",
        "notes.no_notes", "notes.create_first", "notes.no_title",
        "search.tips", "search.tip1", "search.tip2", "search.tip3",
        "srs.today_review", "srs.new_cards", "srs.learning", "srs.mature",
        "srs.create_from_notes", "srs.repetitions", "srs.streak_days",
        "srs.start_review", "srs.no_cards_yet", "srs.create_hint",
        "srs.interval", "srs.days", "srs.delete_card_confirm", "srs.delete_failed",
        "security.enter_current_pin",
        "wiki.no_zim_loaded", "wiki.loaded_zims", "wiki.add_btn", "wiki.add_hint",
        "wiki.name", "wiki.path", "wiki.action",
        "wiki.zim_added", "wiki.no_new_zim", "wiki.refresh_failed",
        "wiki.enter_path", "wiki.add_failed", "wiki.remove_confirm",
        "wiki.zim_removed", "wiki.remove_failed", "wiki.open_folder_msg",
        // 공통
        "app.name", "app.tagline",
        "nav.notes", "nav.search", "nav.wiki",

        // 노트 목록
        "notes.title", "notes.export", "notes.import",
        "notes.no_notes", "notes.create_first", "notes.no_title",
        "notes.edit", "notes.delete", "notes.delete_confirm", "notes.encrypted",
        "notes.find_duplicates", "notes.select_all", "notes.selected",
        "notes.create_package", "notes.duplicates_title", "notes.no_duplicates",
        "notes.export_package", "notes.import_package",
        "notes.package_title", "notes.package_title_placeholder",
        "notes.package_author", "notes.package_author_placeholder",
        "notes.package_description", "notes.package_description_placeholder",
        "notes.drop_file", "notes.or", "notes.select_file",
        "editor.title_placeholder", "editor.content_placeholder",
        "editor.tags", "editor.tags_placeholder", "editor.edit_note",
        "notes.created", "notes.updated",
        "srs.create_cards", "srs.cards_created", "srs.no_cards_extracted",

        // 메인 화면
        "home.notes", "home.streak", "home.day", "home.quick_start",
        "home.new_note", "home.note_list", "home.split_view",
        "home.srs_review", "home.search", "home.zim_manage", "home.security",
        "home.shortcuts", "home.shortcut.new_note", "home.shortcut.search", "home.shortcut.help",
        
        // 에디터
        "editor.title_placeholder", "editor.content_placeholder",
        "editor.tags", "editor.tags_placeholder",
        "editor.encrypt", "editor.focus", "editor.fullscreen", "editor.save",
        "editor.saved", "editor.auto_saved", "editor.changed", "editor.words",
        "editor.save_complete", "editor.encrypt_on", "editor.encrypt_off",
        "editor.pin_required", "editor.pin_locked",
        
        // 노트 목록
        "notes.title", "notes.export", "notes.import",
        "notes.no_notes", "notes.create_first",
        "notes.edit", "notes.delete", "notes.delete_confirm", "notes.encrypted",
        
        // 검색
        "search.title", "search.placeholder", "search.button",
        "search.results", "search.no_results", "search.try_different",
        
        // 위키
        "wiki.search", "wiki.manage", "wiki.status", "wiki.loaded",
        "wiki.directory", "wiki.directory_hint", "wiki.refresh", "wiki.open_folder",
        "wiki.add", "wiki.add_placeholder", "wiki.loaded_files", "wiki.remove",
        "wiki.no_zim", "wiki.no_zim_hint",
        
        // SRS
        "srs.title", "srs.cards", "srs.streak", "srs.show_answer",
        "srs.again", "srs.hard", "srs.good", "srs.easy",
        "srs.complete", "srs.no_cards", "srs.progress",
        
        // 보안
        "security.title", "security.pin_not_set", "security.pin_enabled",
        "security.locked", "security.pin_input", "security.set_pin",
        "security.remove_pin", "security.lock", "security.unlock",
        "security.wrong_pin", "security.pin_set_success", "security.pin_removed",
        "security.unlocked", "security.pin_min_length",
        
        // 백업
        "backup.title", "backup.info", "backup.now",
        "backup.complete", "backup.no_changes",
        
        // 공통 버튼
        "common.confirm", "common.cancel", "common.close",
        "common.loading", "common.error", "common.success",
    ];
    
    let mut map = HashMap::new();
    for key in keys {
        map.insert(key.to_string(), tr.get(key).to_string());
    }
    map
}
