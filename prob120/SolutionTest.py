
from unittest import TestCase

from Solution import Solution


class SolutionTest(TestCase):

    def test_sum(self):
        self.assertEqual(11, Solution().minimumTotal(
            [[2], [3, 4], [6, 5, 7], [4, 1, 8, 3]]))
        self.assertEqual(-10, Solution().minimumTotal([[-10]]))
