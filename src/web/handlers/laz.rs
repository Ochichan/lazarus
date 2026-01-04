//! .laz 패키지 핸들러

use axum::{
    extract::{Multipart, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use uuid::Uuid;

use crate::error::{LazarusError, Result};
use crate::laz::{LazPackage, NoteContent, Curriculum, Chapter};
use crate::web::state::AppState;

/// POST /api/laz/export - 노트를 .laz로 내보내기
pub async fn export_package(
    State(state): State<AppState>,
    Json(params): Json<ExportParams>,
) -> Result<Response> {
    let db = state.db.read().await;
    
    // 패키지 생성
    let mut package = LazPackage::new(&params.title, &params.author);
    package.meta.description = params.description.unwrap_or_default();
    package.meta.language = params.language.unwrap_or_else(|| "ko".to_string());
    
    // 선택된 노트들 추가
    let mut note_ids: Vec<String> = Vec::new();
    
    for note_id in &params.note_ids {
        if let Some(note) = db.get(*note_id)? {
            let uuid = Uuid::new_v4().to_string();
            note_ids.push(uuid.clone());
            
            package.add_note(NoteContent {
                id: uuid,
                title: note.title,
                content: note.content,
                tags: note.tags,
                created_at: note.created_at.timestamp(),
                updated_at: note.updated_at.timestamp(),
            });
        }
    }
    
    // 기본 커리큘럼 (단일 챕터)
    package.curriculum = Curriculum {
        chapters: vec![Chapter {
            id: Uuid::new_v4().to_string(),
            title: params.title.clone(),
            notes: note_ids,
            children: Vec::new(),
        }],
    };
    
    drop(db);
    
    // 임시 파일에 저장
    let filename = format!("{}.laz", sanitize_filename(&params.title));
    let temp_path = std::env::temp_dir().join(&filename);
    
    package.export(&temp_path)?;
    
    // 파일 읽어서 응답
    let data = std::fs::read(&temp_path).map_err(LazarusError::Io)?;
    std::fs::remove_file(&temp_path).ok(); // 임시 파일 삭제
    
    Ok((
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "application/x-lazarus"),
            (header::CONTENT_DISPOSITION, &format!("attachment; filename=\"{}\"", filename)),
        ],
        data,
    ).into_response())
}

/// POST /api/laz/import - .laz 파일 가져오기
pub async fn import_package(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<ImportResult>> {
    // 업로드된 파일 받기
    let mut file_data: Option<bytes::Bytes> = None;
    
    while let Ok(Some(field)) = multipart.next_field().await {
        if field.name() == Some("file") {
            if let Ok(bytes) = field.bytes().await {
                file_data = Some(bytes);
            }
            break;
        }
    }
    
    let data = file_data.ok_or_else(|| {
        LazarusError::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, "파일이 없습니다"))
    })?;
    
    // 임시 파일로 저장
    let temp_path = std::env::temp_dir().join(format!("import_{}.laz", Uuid::new_v4()));
    std::fs::write(&temp_path, &data).map_err(LazarusError::Io)?;
    
    // 패키지 읽기
    let package = LazPackage::import(&temp_path)?;
    
    // 무결성 검사
    let verify = package.verify_integrity(&temp_path)?;
    if !verify.is_valid {
        std::fs::remove_file(&temp_path).ok();
        return Err(LazarusError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("파일 손상: {:?}", verify.errors)
        )));
    }
    
    std::fs::remove_file(&temp_path).ok();
    
    // DB에 노트 저장
    let mut db = state.db.write().await;
    let mut search = state.search.write().await;
    let mut imported_count = 0;
    
    for (_, note_content) in package.content {
        let note = crate::db::Note {
            id: 0, // 새 ID 할당됨
            title: note_content.title,
            content: note_content.content,
            tags: note_content.tags,
            created_at: chrono::DateTime::from_timestamp(note_content.created_at, 0)
                .unwrap_or_else(chrono::Utc::now),
            updated_at: chrono::DateTime::from_timestamp(note_content.updated_at, 0)
                .unwrap_or_else(chrono::Utc::now),
            encrypted: false,
        };
        
        let id = db.save(&note, None)?;
        search.index_note(id, &note.title, &note.content, &note.tags)?;
        imported_count += 1;
    }
    
    tracing::info!(
        "패키지 가져오기 완료: {} (노트 {}개, SRS 카드 {}개)",
        package.meta.title,
        imported_count,
        package.srs.len()
    );
    
    Ok(Json(ImportResult {
        success: true,
        title: package.meta.title,
        notes_imported: imported_count,
        cards_imported: package.srs.len(),
        message: format!("{}개의 노트를 가져왔습니다.", imported_count),
    }))
}

/// POST /api/laz/verify - 파일 무결성 검사
pub async fn verify_package(
    mut multipart: Multipart,
) -> Result<Json<VerifyResponse>> {
    let mut file_data: Option<bytes::Bytes> = None;
    
    while let Ok(Some(field)) = multipart.next_field().await {
        if field.name() == Some("file") {
            if let Ok(bytes) = field.bytes().await {
                file_data = Some(bytes);
            }
            break;
        }
    }
    
    let data = file_data.ok_or_else(|| {
        LazarusError::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, "파일이 없습니다"))
    })?;
    
    let temp_path = std::env::temp_dir().join(format!("verify_{}.laz", Uuid::new_v4()));
    std::fs::write(&temp_path, &data).map_err(LazarusError::Io)?;
    
    let package = LazPackage::import(&temp_path)?;
    let verify = package.verify_integrity(&temp_path)?;
    
    std::fs::remove_file(&temp_path).ok();
    
    Ok(Json(VerifyResponse {
        is_valid: verify.is_valid,
        verified_count: verify.verified_count,
        total_count: verify.total_count,
        errors: verify.errors,
        package_info: PackageInfo {
            title: package.meta.title,
            author: package.meta.author,
            version: package.meta.version,
            notes_count: package.content.len(),
            cards_count: package.srs.len(),
        },
    }))
}

#[derive(serde::Deserialize)]
pub struct ExportParams {
    pub title: String,
    pub author: String,
    pub note_ids: Vec<u64>,
    pub description: Option<String>,
    pub language: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ImportResult {
    pub success: bool,
    pub title: String,
    pub notes_imported: usize,
    pub cards_imported: usize,
    pub message: String,
}

#[derive(serde::Serialize)]
pub struct VerifyResponse {
    pub is_valid: bool,
    pub verified_count: usize,
    pub total_count: usize,
    pub errors: Vec<String>,
    pub package_info: PackageInfo,
}

#[derive(serde::Serialize)]
pub struct PackageInfo {
    pub title: String,
    pub author: String,
    pub version: u32,
    pub notes_count: usize,
    pub cards_count: usize,
}

/// 파일명 정리
fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' || c == ' ' { c } else { '_' })
        .collect::<String>()
        .trim()
        .to_string()
}
