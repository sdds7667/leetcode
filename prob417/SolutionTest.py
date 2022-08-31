
from unittest import TestCase

from Solution import Solution


class SolutionTest(TestCase):

    def test_sum(self):
        print()
        self.assertEqual([[0, 4], [1, 3], [1, 4], [2, 2], [3, 0], [3, 1], [4, 0]], Solution().pacificAtlantic(
            [[1, 2, 2, 3, 5], [3, 2, 3, 4, 4], [2, 4, 5, 3, 1], [6, 7, 1, 4, 5], [5, 1, 1, 2, 4]]))

        self.assertEqual([[0, 0]], Solution().pacificAtlantic([[1]]))
