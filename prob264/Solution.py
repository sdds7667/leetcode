from typing import List
from heapq import *

class Solution:
    def nthUglyNumber(self, n: int) -> int:
        un = [1]
        n -= 1
        s = set()
        while n != 0:
            n -= 1
            i = heappop(un)
            k = i * 2
            if (k not in s):
                s.add(k)
                heappush(un, k)
            
            k = i * 3
            if (k not in s):
                s.add(k)
                heappush(un, k)
            k = i * 5
            if (k not in s):
                s.add(k)
                heappush(un, k)
        return un[n]


