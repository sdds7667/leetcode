from typing import List
import numpy as np

class Solution:
    def minCut(self, s: str) -> int:
        
        n = len(s)
        cuts = [x for x in range(-1, n)]

        for i in range(n):

            odd, even = 0, 0

            while i - odd >= 0 and i + odd < n and s[i-odd] == s[i+odd]:
                cuts[i+odd+1] = min(cuts[i+odd+1],cuts[i-odd]+1)
                odd += 1

            while i - even >= 0 and i + even + 1 < n and s[i-even] == s[i+even+1]:
                cuts[i+even+2] = min(cuts[i+even+2], cuts[i-even] + 1)
                even += 1

        return cuts[-1]

if __name__ == '__main__':
    print(Solution().minCut("aab"))
    print(Solution().minCut("baab"))
    print(Solution().minCut("baaa"))
    print(Solution().minCut("baa"))