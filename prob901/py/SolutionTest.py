
from unittest import TestCase

from Solution import Solution


class SolutionTest(TestCase):

    def test_sum(self):
        self.assertEqual(3, Solution().sum(1, 2))
