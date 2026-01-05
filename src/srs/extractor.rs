//! 노트/위키에서 플래시카드 자동 추출

use super::{Card, CardType};
use regex::Regex;

/// 노트 콘텐츠에서 카드 추출
pub fn extract_cards_from_note(note_id: u64, title: &str, content: &str) -> Vec<Card> {
    let mut cards = Vec::new();

    // 1. 빈칸(Cloze) 패턴: {{답}}
    cards.extend(extract_cloze(note_id, content));

    // 2. Q&A 패턴: ## 질문?\n답변
    cards.extend(extract_qa_headers(note_id, content));

    // 3. 정의 패턴: - 용어: 정의 또는 **용어**: 정의
    cards.extend(extract_definitions(note_id, content));

    // 4. 제목으로 기본 카드 (다른 카드 없으면)
    if cards.is_empty() && !title.is_empty() && !content.is_empty() {
        let preview: String = content.chars().take(200).collect();
        cards.push(Card {
            id: 0,
            card_type: CardType::Basic,
            question: format!("{}에 대해 설명하세요.", title.trim()),
            answer: preview.trim().to_string(),
            source_note_id: Some(note_id),
            source_wiki_url: None,
            hints: vec![],
            tags: vec![],
            srs: Default::default(),
            created_at: chrono::Utc::now(),
        });
    }

    cards
}

/// 빈칸(Cloze) 추출: {{답}} 패턴
fn extract_cloze(note_id: u64, content: &str) -> Vec<Card> {
    let mut cards = Vec::new();
    let re = Regex::new(r"\{\{([^}]+)\}\}").unwrap();

    for line in content.lines() {
        if re.is_match(line) {
            let answer = re
                .captures(line)
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().to_string())
                .unwrap_or_default();

            let question = re.replace_all(line, "[...]").to_string();

            if !answer.is_empty() {
                cards.push(Card {
                    id: 0,
                    card_type: CardType::Cloze,
                    question: question.trim().to_string(),
                    answer,
                    source_note_id: Some(note_id),
                    source_wiki_url: None,
                    hints: vec![],
                    tags: vec!["cloze".to_string()],
                    srs: Default::default(),
                    created_at: chrono::Utc::now(),
                });
            }
        }
    }

    cards
}

/// Q&A 헤더 추출: ## 질문?\n답변
fn extract_qa_headers(note_id: u64, content: &str) -> Vec<Card> {
    let mut cards = Vec::new();
    let re = Regex::new(r"(?m)^#{1,4}\s*(.+\?)\s*$").unwrap();

    let lines: Vec<&str> = content.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        if let Some(caps) = re.captures(line) {
            let question = caps
                .get(1)
                .map(|m| m.as_str().trim().to_string())
                .unwrap_or_default();

            // 다음 줄들에서 답변 추출 (다음 헤더 전까지)
            let mut answer_lines = Vec::new();
            for j in (i + 1)..lines.len() {
                if lines[j].starts_with('#') {
                    break;
                }
                let trimmed = lines[j].trim();
                if !trimmed.is_empty() {
                    answer_lines.push(trimmed);
                }
            }

            let answer = answer_lines.join(" ");

            if !question.is_empty() && !answer.is_empty() {
                cards.push(Card {
                    id: 0,
                    card_type: CardType::Basic,
                    question,
                    answer,
                    source_note_id: Some(note_id),
                    source_wiki_url: None,
                    hints: vec![],
                    tags: vec!["qa".to_string()],
                    srs: Default::default(),
                    created_at: chrono::Utc::now(),
                });
            }
        }
    }

    cards
}

/// 정의 추출: - 용어: 정의 또는 **용어**: 정의
fn extract_definitions(note_id: u64, content: &str) -> Vec<Card> {
    let mut cards = Vec::new();

    // 패턴 1: - 용어: 정의
    let re1 = Regex::new(r"(?m)^[-*]\s*(.+?):\s*(.+)$").unwrap();
    // 패턴 2: **용어**: 정의
    let re2 = Regex::new(r"\*\*(.+?)\*\*:\s*(.+)").unwrap();

    for caps in re1.captures_iter(content) {
        let term = caps.get(1).map(|m| m.as_str().trim()).unwrap_or("");
        let definition = caps.get(2).map(|m| m.as_str().trim()).unwrap_or("");

        if !term.is_empty() && !definition.is_empty() && definition.len() > 5 {
            cards.push(Card {
                id: 0,
                card_type: CardType::Definition,
                question: format!("{}의 정의는?", term),
                answer: definition.to_string(),
                source_note_id: Some(note_id),
                source_wiki_url: None,
                hints: vec![],
                tags: vec!["definition".to_string()],
                srs: Default::default(),
                created_at: chrono::Utc::now(),
            });
        }
    }

    for caps in re2.captures_iter(content) {
        let term = caps.get(1).map(|m| m.as_str().trim()).unwrap_or("");
        let definition = caps.get(2).map(|m| m.as_str().trim()).unwrap_or("");

        if !term.is_empty() && !definition.is_empty() && definition.len() > 5 {
            cards.push(Card {
                id: 0,
                card_type: CardType::Definition,
                question: format!("{}이란?", term),
                answer: definition.to_string(),
                source_note_id: Some(note_id),
                source_wiki_url: None,
                hints: vec![],
                tags: vec!["definition".to_string()],
                srs: Default::default(),
                created_at: chrono::Utc::now(),
            });
        }
    }

    cards
}

/// 위키 문서에서 카드 추출
pub fn extract_cards_from_wiki(url: &str, title: &str, content: &str) -> Vec<Card> {
    let mut cards = Vec::new();

    // HTML 태그 제거
    let re_html = Regex::new(r"<[^>]+>").unwrap();
    let clean_content = re_html.replace_all(content, " ").to_string();

    // 첫 문단 추출 (첫 200자)
    let first_para: String = clean_content
        .lines()
        .filter(|l| !l.trim().is_empty())
        .take(3)
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(300)
        .collect();

    if !title.is_empty() && !first_para.is_empty() {
        // 기본 카드: "X란 무엇인가?"
        cards.push(Card {
            id: 0,
            card_type: CardType::Definition,
            question: format!("{}(이)란 무엇인가?", title),
            answer: first_para.trim().to_string(),
            source_note_id: None,
            source_wiki_url: Some(url.to_string()),
            hints: vec![],
            tags: vec!["wiki".to_string()],
            srs: Default::default(),
            created_at: chrono::Utc::now(),
        });
    }

    cards
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cloze_extraction() {
        let content = "뉴턴의 제2법칙: F = {{ma}}\n에너지 공식: E = {{mc²}}";
        let cards = extract_cloze(1, content);
        assert_eq!(cards.len(), 2);
        assert_eq!(cards[0].answer, "ma");
        assert_eq!(cards[1].answer, "mc²");
    }

    #[test]
    fn test_qa_extraction() {
        let content = "## 관성이란 무엇인가?\n물체가 현재 상태를 유지하려는 성질";
        let cards = extract_qa_headers(1, content);
        assert_eq!(cards.len(), 1);
        assert!(cards[0].question.contains("관성"));
    }

    #[test]
    fn test_definition_extraction() {
        let content = "- 관성: 물체가 현재 상태를 유지하려는 성질\n- 가속도: 속도의 변화율";
        let cards = extract_definitions(1, content);
        assert_eq!(cards.len(), 2);
    }
}
