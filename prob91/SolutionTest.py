
from unittest import TestCase

from Solution import Solution


class SolutionTest(TestCase):

    def test_leet_code(self):
        list(map(lambda x: self.assertEqual(x[1], Solution().numDecodings(x[0])), [
            ("12", 2),
            ("226", 3),
            ("06", 0),
            ("101010", 1)
        ]))
        
