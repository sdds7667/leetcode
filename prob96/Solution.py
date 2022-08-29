from typing import List


class Solution:
    def __init__(self):
        self.dp = [None] * 20
        self.dp[0] = 1
        self.dp[1] = 1 
        
    def numTrees(self, n: int) -> int:
        if (self.dp[n] != None): return self.dp[n]
        k = 0
        for i in range(1, n + 1):
            k += self.numTrees(i - 1) * self.numTrees(n - i)
        self.dp[n] = k
        return k