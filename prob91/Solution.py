from typing import List


class Solution:
    def numDecodings(self, s: str) -> int:
        """dp like climbing stairs"""
        n = len(s)
        c = 0
        c1 = 1
        c2 = 1
        for i in reversed(range(n)):
            c = 0 if (s[i] == '0') else (c1 + (0 if i == n - 1 else ((c2) if ( 10 <= int(s[i:i+2]) <= 26) else 0)))
            c2 = c1
            c1 = c
        return c
