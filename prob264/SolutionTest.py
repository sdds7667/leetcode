
from unittest import TestCase

from Solution import Solution


class SolutionTest(TestCase):

    def test_sum(self):
        self.assertEqual(12, Solution().nthUglyNumber(10))
        self.assertEqual(1, Solution().nthUglyNumber(1))
