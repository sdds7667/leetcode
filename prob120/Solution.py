from typing import List


class Solution:
    def minimumTotal(self, triangle: List[List[int]]) -> int:
        for i in range(1, len(triangle)):
            p = triangle[i-1]
            t = triangle[i]
            lt = len(t)
            for j in range(lt):
                t[j] += p[j] if j == 0 else (p[j - 1]
                                             if j == lt-1 else min(p[j], p[j-1]))
        return min(triangle[-1])
