use regex::Regex;
use std::sync::LazyLock;

pub static GREETING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?ix)
        ^\s*
        (?:
            assalomu\s+alaykum
            | assalom
            | sal+o+m+
            | hello
            | hi+
            | hey
            | privet
            | привет
            | здравствуйте
        )
        (?:\b|[!,.?])*
        (?:\s|$)
        ",
    )
    .unwrap()
});
