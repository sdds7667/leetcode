
from unittest import TestCase

from Solution import Solution


class SolutionTest(TestCase):


    def test_leetcode_1(self): 
        self.assertEqual(3, Solution().maxProfit([1,2,3,0,2]))

    def test_leetcode_2(self): 
        self.assertEqual(0, Solution().maxProfit([1]))

    def test_own(self):
        self.assertEqual(11, Solution().maxProfit([2, 5, 1, 5, 3, 7, 2, 9, 9, 9]))

    def test_own_1(self):
        self.assertEqual(30, Solution().maxProfit([18, 13, 19,  2,  6,  4,  5,  7, 16,  7, 11,  8,  3,  7, 15]))
