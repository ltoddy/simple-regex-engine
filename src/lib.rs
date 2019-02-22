pub mod regex {
    fn substring(raw: &str, start: Option<usize>, end: Option<usize>) -> String {
        match (start, end) {
            (None, None) => raw.chars().collect::<String>(),
            (Some(start), None) => raw.chars().skip(start).collect::<String>(),
            (None, Some(end)) => raw.chars().take(end).collect::<String>(),
            (Some(start), Some(end)) => raw
                .chars()
                .skip(start)
                .take(end - start)
                .collect::<String>(),
        }
    }

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

    fn match_question<P, T>(pattern: P, text: T) -> bool
    where
        P: Into<String>,
        T: Into<String>,
    {
        let pattern = pattern.into();
        let text = text.into();

        (match_one(
            substring(&pattern, None, Some(1)),
            substring(&text, None, Some(1)),
        ) && matches(
            substring(&pattern, Some(2), None),
            substring(&text, Some(1), None),
        )) || matches(substring(&pattern, Some(2), None), text)
    }

    fn match_star<P, T>(pattern: P, text: T) -> bool
    where
        P: Into<String>,
        T: Into<String>,
    {
        let pattern = pattern.into();
        let text = text.into();

        (match_one(
            substring(&pattern, None, Some(1)),
            substring(&text, None, Some(1)),
        ) && matches(pattern.clone(), substring(&text, Some(1), None)))
            || matches(substring(&pattern, Some(2), None), text)
    }

    #[doc("相同长度的字符串匹配,(由于match是关键字,这里用matches命名)")]
    pub fn matches<P, T>(pattern: P, text: T) -> bool
    where
        P: Into<String>,
        T: Into<String>,
    {
        let pattern = pattern.into();
        let text = text.into();

        if pattern.is_empty() || pattern.starts_with('$') && text == "" {
            true
        } else if substring(&pattern, Some(1), None).starts_with('?') {
            match_question(pattern, text)
        } else if substring(&pattern, Some(1), None).starts_with('*') {
            match_star(pattern.clone(), text.clone())
        } else {
            match_one(
                substring(&pattern, None, Some(1)),
                substring(&text, None, Some(1)),
            ) && matches(
                substring(&pattern, Some(1), None),
                substring(&text, Some(1), None),
            )
        }
    }

    pub fn search<P, T>(pattern: P, text: T) -> bool
    where
        P: Into<String>,
        T: Into<String>,
    {
        let pattern = pattern.into();
        let text = text.into();

        if pattern.starts_with('^') {
            matches(substring(&pattern, Some(1), None), text)
        } else {
            matches(format!(".*{}", pattern), text)
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::regex::{match_one, matches, search};

    #[test]
    pub fn test_match_one() {
        assert_eq!(match_one(r#"a"#, "a"), true);
        assert_eq!(match_one(r#"."#, "z"), true);
        assert_eq!(match_one(r#""#, "g"), true);
        assert_eq!(match_one(r#"a"#, "b"), false);
        assert_eq!(match_one(r#"p"#, ""), false);
    }

    #[test]
    pub fn test_matches() {
        assert_eq!(matches(r#"a.c"#, "abc"), true);
        assert_eq!(matches(r#"a.c$"#, "abc"), true);
    }

    #[test]
    pub fn test_search() {
        assert_eq!(search(r#"^abc"#, "abc"), true);
        assert_eq!(search(r#"^abcd"#, "abcd"), true);
        assert_eq!(search(r#"bc"#, "abcd"), true);

        assert_eq!(search(r#"ab?c"#, "ac"), true);
        assert_eq!(search(r#"ab?c"#, "abc"), true);
        assert_eq!(search(r#"a?b?c?"#, "abc"), true);
        assert_eq!(search(r#"a?b?c?"#, ""), true);

        assert_eq!(search(r#"a*"#, ""), true);
        assert_eq!(search(r#"a*"#, "aaaaaaaaa"), true);
        assert_eq!(search(r#"a*b"#, "aaaaaaaaaaab"), true);
    }
}
