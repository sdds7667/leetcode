
from unittest import TestCase

from Solution import Solution


class SolutionTest(TestCase):

    def test_sum(self):
        self.assertEqual(3, Solution().numberOfArithmeticSlices([1, 2,3,4]))
        self.assertEqual(0, Solution().numberOfArithmeticSlices([1]))
