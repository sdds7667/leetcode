from typing import List


class Solution:
    def countSpecialSubsequences(self, nums: List[int]) -> int:
        x = 0
        for i in range(nums):
            x += self.recursive_solution(nums, -1, i)
        return x

    def recursive_solution(self, nums: List[int], highest: int, i: int) -> int:
        # take the current number
        if highest == 2: return 1
        x = 0
        for j in range(i, len(nums)):
            if nums[j] == highest + 1:
                x += self.recursive_solution(nums, highest + 1, j)
        return x

