
from unittest import TestCase

from Solution import Solution


class SolutionTest(TestCase):

    def test_sum(self):
        self.assertEqual(7, Solution().minPathSum([[1,3,1],[1,5,1],[4,2,1]]))
        self.assertEqual(12, Solution().minPathSum([[1,2,3],[4,5,6]]))