#[doc("单个字符的匹配")]
pub fn match_one<S>(pattern: S, text: S) -> bool
where
    S: Into<String>,
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

#[cfg(test)]
pub mod test {
    use super::match_one;

    #[test]
    pub fn test_match_one() {
        assert_eq!(match_one("a", "a"), true);
        assert_eq!(match_one(".", "z"), true);
        assert_eq!(match_one("", "g"), true);
        assert_eq!(match_one("a", "b"), false);
        assert_eq!(match_one("p", ""), false);
    }
}
