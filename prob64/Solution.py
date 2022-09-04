from typing import List


class Solution:
    def minPathSum(self, grid: List[List[int]]) -> int:
        r = len(grid)
        c = len(grid[0])
        dp = [[0 for i in range(c)] for i in range(r)]
        prev = grid[r-1][c-1]
        dp[r-1][c-1] = prev

        j = c-2
        while j >= 0:
            prev += grid[r-1][j] 
            dp[r-1][j] = prev
            j -= 1

        i = r - 2
        prev = dp[r-1][c-1]
        while i >= 0:
            prev += grid[i][c-1]
            dp[i][c-1] = prev

        i = r - 2
        while(i >= 0):
            j = c-2
            while(j >= 0):
                dp[i][j] = max(dp[i+1][j], dp[i][j+1]) + grid[i][j]
                j -= 1
            i -= 1
        
        return dp[0][0]
