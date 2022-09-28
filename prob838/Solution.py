import enum
from typing import List


class Solution:
    def pushDominoes(self, dominoes: str) -> str:
        dl = [0] * len(dominoes)
        cdl = 100000
        j = len(dominoes) - 1
        for i in reversed(dominoes):
            if i == 'L':
                cdl = 0
            elif i == 'R':
                cdl = 100000
            elif i == '.':
                cdl = min(cdl + 1, 100000)
            dl[j] = cdl
            j -= 1
        
        r = []
        cdr = 100000
        for ind, i in enumerate(dominoes):
            if i == 'R':
                cdr = 0
            elif i == 'L':
                cdr = 100000
            else:
                cdr = min(cdr+1, 100000)
            if dl[ind] == cdr:
                r.append('.')
            elif dl[ind] > cdr:
                r.append('R')
            else:
                r.append('L')

        return "".join(r)
