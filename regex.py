import unittest


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


def match(pattern: str, text: str) -> bool:
    """
    匹配同样长度的字符串
    """
    if not pattern:
        return True
    else:
        return match_one(pattern[0], text[0]) and match(pattern[1:], text[1:])


class RegexTestCase(unittest.TestCase):
    def test_match_one(self):
        self.assertTrue(match_one('a', 'a'))
        self.assertTrue(match_one('.', 's'))
        self.assertTrue(match_one('', 'a'))
        self.assertFalse(match_one('a', 'b'))
        self.assertFalse(match_one('a', ''))

    def test_match(self):
        self.assertTrue("a.c", "abc")
