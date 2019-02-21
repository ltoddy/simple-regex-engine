pub mod regex {
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
            true
        } else {
            let pattern = pattern.chars();
            let text = text.chars();

            match_one(
                pattern.clone().take(1).collect::<String>(),
                text.clone().take(1).collect::<String>(),
            ) && matches(
                pattern.clone().skip(1).collect::<String>(),
                text.clone().skip(1).collect::<String>(),
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
            matches(pattern.chars().skip(1).collect::<String>(), text)
        } else {
            let text = text.chars();
            text.clone()
                .enumerate()
                .any(|(i, _)| matches(pattern.clone(), text.clone().skip(i).collect::<String>()))
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
    }
}
