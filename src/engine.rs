#[doc("单个字符的匹配")]
pub fn match_one<P, T>(pattern: P, text: T) -> bool
where
    P: Into<String>,
    T: Into<String>,
{
    let pattern = pattern.into();
    let text = text.into();

    if pattern.is_empty() {
        // 任何text都可匹配空的pattern
        return true;
    }

    if text.is_empty() {
        // 如果文本时空的, 就返回false
        return false;
    }

    if pattern == "." {
        return true;
    }

    pattern == text
}

#[doc("相同长度的字符串匹配,(由于match是关键字,这里用matches命名)")]
pub fn matches<P, T>(pattern: P, text: T) -> bool
where
    P: Into<String>,
    T: Into<String>,
{
    let pattern = pattern.into();
    let text = text.into();

    if pattern.is_empty() {
        return true;
    }

    if pattern == "$" && text.is_empty() {
        return true;
    } else {
        let pattern = pattern.chars();
        let text = text.chars();

        return match_one(
            pattern.clone().take(1).collect::<String>(),
            text.clone().take(1).collect::<String>(),
        ) && matches(
            pattern.clone().skip(1).collect::<String>(),
            text.clone().skip(1).collect::<String>(),
        );
    }
}

#[cfg(test)]
pub mod test {
    use super::{match_one, matches};

    #[test]
    pub fn test_match_one() {
        assert_eq!(match_one("a", "a"), true);
        assert_eq!(match_one(".", "z"), true);
        assert_eq!(match_one("", "g"), true);
        assert_eq!(match_one("a", "b"), false);
        assert_eq!(match_one("p", ""), false);
    }

    #[test]
    pub fn test_matches() {
        assert_eq!(matches(r#"a.c"#, "abc"), true);
        assert_eq!(matches(r#"a.c$"#, "abc"), true);
    }
}
