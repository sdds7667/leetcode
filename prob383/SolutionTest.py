
from unittest import TestCase

from Solution import Solution


class SolutionTest(TestCase):

    def test_leetcode(self):
        self.assertFalse(Solution().canConstruct("a", "b"))
        self.assertFalse(Solution().canConstruct("aa", "ab"))
        self.assertTrue(Solution().canConstruct("aa", "aab"))
