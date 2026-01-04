//! 언어 설정 핸들러

use axum::{
    extract::State,
    response::Redirect,
    Form,
};
use serde::Deserialize;

use crate::i18n::Lang;
use crate::error::Result;
use crate::web::state::AppState;

#[derive(Deserialize)]
pub struct LangForm {
    pub lang: String,
}

/// POST /api/lang - 언어 변경
pub async fn set_lang(
    State(state): State<AppState>,
    Form(form): Form<LangForm>,
) -> Result<Redirect> {
    let lang = match form.lang.as_str() {
        "en" => Lang::En,
        "ko" => Lang::Ko,
        _ => Lang::En,
    };
    
    state.set_lang(lang).await;
    tracing::info!("언어 변경: {:?}", lang);
    
    // Referer가 있으면 그 페이지로, 없으면 메인으로
    Ok(Redirect::to("/"))
}
