//! 한국어 번역

use super::Translations;
use std::collections::HashMap;

pub fn translations() -> Translations {
    let mut data = HashMap::new();

    // 공통
    data.insert("app.name", "Lazarus");
    data.insert("app.tagline", "오프라인에서도 작동하는 개인 지식 관리");
    data.insert("nav.notes", "노트");
    data.insert("nav.search", "검색");
    data.insert("nav.wiki", "위키");

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
    data.insert("editor.edit_note", "노트 수정");
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
    data.insert("notes.no_title", "제목 없음");
    data.insert("notes.edit", "편집");
    data.insert("notes.delete", "삭제");
    data.insert("notes.delete_confirm", "정말 삭제하시겠습니까?");
    data.insert("notes.encrypted", "암호화됨");
    data.insert("notes.created", "생성");
    data.insert("notes.updated", "수정");
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
    data.insert(
        "notes.package_description_placeholder",
        "패키지 설명 (선택)",
    );
    data.insert("notes.drop_file", ".laz 파일을 여기에 드롭하세요");
    data.insert("notes.or", "또는");
    data.insert("notes.select_file", "파일 선택");

    // 검색
    data.insert("search.title", "검색");
    data.insert("search.placeholder", "검색어 입력...");
    data.insert("search.button", "검색");
    data.insert("search.results", "검색 결과");
    data.insert("search.no_results", "검색 결과가 없습니다");
    data.insert("search.try_different", "다른 검색어를 시도해보세요");
    data.insert("search.tips", "검색 팁");
    data.insert(
        "search.tip1",
        "여러 단어를 입력하면 모두 포함된 결과를 찾습니다",
    );
    data.insert("search.tip2", "노트와 위키백과를 동시에 검색합니다");
    data.insert("search.tip3", "제목에 일치하는 결과가 먼저 표시됩니다");

    // 위키
    data.insert("wiki.search", "위키 검색");
    data.insert("wiki.manage", "ZIM 관리");
    data.insert("wiki.status", "현황");
    data.insert("wiki.loaded", "개 로드됨");
    data.insert("wiki.directory", "ZIM 디렉토리");
    data.insert(
        "wiki.directory_hint",
        "이 폴더에 .zim 파일을 복사 후 새로고침",
    );
    data.insert("wiki.refresh", "새로고침");
    data.insert("wiki.open_folder", "폴더 열기");
    data.insert("wiki.add", "ZIM 추가");
    data.insert("wiki.add_placeholder", "ZIM 파일 경로 입력...");
    data.insert("wiki.loaded_files", "로드된 ZIM 파일");
    data.insert("wiki.remove", "제거");
    data.insert("wiki.no_zim", "ZIM 파일 없음");
    data.insert(
        "wiki.no_zim_hint",
        "위키백과를 사용하려면 ZIM 파일을 추가하세요",
    );
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
    data.insert(
        "wiki.open_folder_msg",
        "파일 탐색기에서 직접 폴더를 열어주세요:",
    );

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
    data.insert("srs.today_review", "오늘 복습");
    data.insert("srs.new_cards", "새 카드");
    data.insert("srs.learning", "학습 중");
    data.insert("srs.mature", "완료");
    data.insert("srs.create_from_notes", "노트에서 카드 만들기");
    data.insert("srs.create_cards", "카드 생성");
    data.insert("srs.cards_created", "개 카드가 생성되었습니다");
    data.insert("srs.no_cards_extracted", "추출할 카드가 없습니다");
    data.insert("srs.repetitions", "반복");
    data.insert("srs.streak_days", "연속");
    data.insert("srs.start_review", "복습하기");
    data.insert("srs.no_cards_yet", "아직 카드가 없습니다.");
    data.insert(
        "srs.create_hint",
        "노트를 열고 \"카드 생성\" 버튼을 클릭하세요.",
    );
    data.insert("srs.interval", "간격");
    data.insert("srs.days", "일");
    data.insert("srs.delete_card_confirm", "이 카드를 삭제하시겠습니까?");
    data.insert("srs.delete_failed", "삭제 실패");

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
    data.insert("security.enter_current_pin", "현재 PIN을 입력하세요");

    // 백업
    data.insert("backup.title", "백업");
    data.insert("backup.info", "백업 정보");
    data.insert("backup.now", "지금 백업");
    data.insert("backup.complete", "백업 완료");
    data.insert("backup.no_changes", "변경사항 없음 (백업 스킵)");

    // 스플릿뷰
    data.insert("split.select_note", "왼쪽에서 노트를 선택하세요");

    // 공통 버튼
    data.insert("common.confirm", "확인");
    data.insert("common.cancel", "취소");
    data.insert("common.close", "닫기");
    data.insert("common.loading", "로딩 중...");
    data.insert("common.error", "오류가 발생했습니다");
    data.insert("common.success", "성공");

    // Settings
    data.insert("settings.title", "설정");
    data.insert("settings.language", "언어");
    data.insert("settings.language_desc", "원하는 언어를 선택하세요");
    data.insert("settings.stats", "통계");
    data.insert("settings.version", "버전");
    data.insert("settings.storage", "저장소");
    data.insert("settings.about", "정보");

    // Knowledge Graph & Footer
    data.insert("home.knowledge_graph", "지식 그래프");
    data.insert("app.footer", "모두를 위한 지식");

    // Knowledge Graph & Footer
    data.insert("home.knowledge_graph", "지식 그래프");
    data.insert("app.footer", "모두를 위한 지식");

    Translations::new(data)
}
