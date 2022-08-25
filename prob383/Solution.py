from typing import List
from collections import Counter
import numpy as np


# Solution using numpy arrays: ~430ms
# class Solution:
#     def canConstruct(self, ransomNote: str, magazine: str) -> bool:
#         mc = np.zeros((150))
#         rn = np.zeros((150))
        
#         for i in magazine:
#             mc[ord(i)] += 1
        
#         for i in ransomNote:
#             rn[ord(i)] += 1
        
#         return len(np.where((mc-rn) < 0)[0]) == 0

# Solution using Counter ~220ms
class Solution:
    def canConstruct(self, ransomNote: str, magazine: str) -> bool:
        mc = Counter(magazine)
        rc = Counter(ransomNote)

        for key, value in rc.items():
            if value > mc[key]:
                return False
        return True



