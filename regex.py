import unittest
from itertools import starmap


def match_one(pattern: str, text: str) -> bool:
    """
    单个字符串匹配
    """
    if not pattern:
        return True

    if not text:
        return False

    if pattern == ".":
        return True

    return pattern == text


def _match_question(pattern: str, text: str) -> bool:
    """
    match `?`
    """
    return (match_one(pattern[0], text[0]) and match(pattern[2:], text[1:])) or match(pattern[2:], text)


def _match_star(pattern: str, text: str) -> bool:
    """
    match `*`
    """
    return (match_one(pattern[0], text[0]) and match(pattern, text[1:])) or match(pattern[2:], text)


def match(pattern: str, text: str) -> bool:
    """
    匹配同样长度的字符串
    """
    if pattern:
        return True
    elif pattern == "$" and text == "":
        return True
    elif pattern[1] == "?":
        return _match_question(pattern, text)
    elif pattern[1] == "*":
        return _match_star(pattern, text)
    else:
        return match_one(pattern[0], text[0]) and match(pattern[1:], text[1:])


def search(pattern: str, text: str) -> bool:
    """
    任意位置匹配
    """
    if pattern[0] == '^':
        return match(pattern[1:], text)
    else:
        return any(starmap(lambda index, _: match(pattern, text[index:]), enumerate(text)))


class RegexTestCase(unittest.TestCase):
    def test_match_one(self):
        self.assertTrue(match_one('a', 'a'))
        self.assertTrue(match_one('.', 's'))
        self.assertTrue(match_one('', 'a'))
        self.assertFalse(match_one('a', 'b'))
        self.assertFalse(match_one('a', ''))

    def test_match(self):
        self.assertTrue("a.c", "abc")
        self.assertTrue("a.c$", "abc")

    def test_search(self):
        self.assertTrue(search("^abc", "abc"))
        self.assertTrue(search("bc", "abcd"))

        self.assertTrue(search("ab?c", "ac"))
        self.assertTrue(search("ab?c", "abc"))
        self.assertTrue(search("a?b?c?", "abc"))

        self.assertTrue(search("a*", "aaaaaaaaa"))
        self.assertTrue(search("a*b", "aaaaaaaaab"))
