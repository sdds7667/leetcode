
from unittest import TestCase

from Solution import Solution


class SolutionTest(TestCase):

    # def test_sum(self):
    #     self.assertEqual("RR.L", Solution().pushDominoes("RR.L"))

    def test_leetcode(self):
        self.assertEqual("LL.RR.LLRRLL..",
                         Solution().pushDominoes(".L.R...LR..L.."))
