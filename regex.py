import unittest


def match_one(pattern: str, text: str) -> bool:
    if not pattern:
        return True

    if not text:
        return False

    if pattern == ".":
        return True

    return pattern == text


class RegexTestCase(unittest.TestCase):
    def test_match_one(self):
        self.assertTrue(match_one('a', 'a'))
        self.assertTrue(match_one('.', 's'))
        self.assertTrue(match_one('', 'a'))
        self.assertFalse(match_one('a', 'b'))
        self.assertFalse(match_one('a', ''))
