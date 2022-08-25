from typing import List


# Recursive solution for the problem

class RecurseSolution:
    def maxProfit(self, prices: List[int]) -> int:
        if (len(prices) == 1): return 0
        m = 0
        for i in range(len(prices)):
            for j in range(i+1, len(prices)):
                m = max(prices[j] - prices[i] + self.maxProfit(prices[j+2:]), m)
        return m

# N^2 solution
class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        n = len(prices)
        dp = [0] * (n + 2) 
        
        for i in reversed(range(n - 1)):
            m = 0
            for j in range(i + 1, n):
                m = max(prices[j] - prices[i] + dp[j+2], dp[j], m)
            dp[i] = m
        
        return dp[0]

if __name__ == "__main__": 
    print(Solution().maxProfit([18, 13, 19,  2,  6,  4,  5,  7, 16,  7, 11,  8,  3,  7, 15]))