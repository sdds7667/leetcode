from typing import List, Dict


class Solution:
    def longestPalindrome(self, words: List[str]) -> int:
        counts : Dict[str, int] = {}
        for i in words:
            counts[i] = 1 + counts.get(i, 0)

        hm = False
        size = 0
        for i in counts:
            lc = counts.get(i, 0)
            if lc == 0:
                continue
            if i[0] == i[1]:
                size += (lc // 2) * 4
                if not hm and size % 2 == 1:
                        hm = True
                        size += 2
                continue
            rc = counts.get(i[::-1], 0)
            if rc == 0:
                if not hm:
                    hm = True
                    size += 2
            else:
                size += 4 * min(lc, rc)
                if lc != rc:
                    if not hm:
                        hm = True
                        size += 2
        
        return size



