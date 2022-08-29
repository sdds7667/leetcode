from typing import List


class Solution:
    def numberOfArithmeticSlices(self, nums: List[int]) -> int:
        if (len(nums) < 3): return 0;
        d = nums[1]-nums[0]
        p = nums[1]
        sl = 2
        tc = 0
        for i in nums[2:]:
            nd = i - p
            p = i
            if nd == d:
                sl += 1
            else:
                d = nd
                if (sl < 3): continue
                tc += ((sl - 2) * (sl - 1)) // 2
                sl = 2

                
        if (sl < 3): return tc
        tc += ((sl - 2) * (sl - 1)) // 2    
        return tc
