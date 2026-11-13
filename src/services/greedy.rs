use crate::config::greedy::GREETING;

pub async fn is_greedy(text: &str) -> bool {
    let text = text.trim();

    if text.is_empty() {
        return false;
    }

    GREETING.is_match(text)
}