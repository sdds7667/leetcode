# This is a sample Python script.
from typing import List


# Press ⌃R to execute it or replace it with your code.
# Press Double ⇧ to search everywhere for classes, files, tool windows, actions, and settings.

class Solution:
    def maxNumOfSubstrings(self, s: str) -> List[str]:
        intervals = {}
        index = 0
        for i in s:
            if i in intervals:
                intervals[i][1] = index
            else:
                intervals[i] = [index, index]
            index += 1

        ls = []
        for c in intervals:
            b, e = intervals[c][0], intervals[c][1]
            tb = b
            i = b
            while i <= e and b == intervals[c][0]:
                b = min(b, fst[s[i]])
                e = max(e, lst[s[i]])
                i += 1
            if b == fst[c]:
                intervals.append((e, b))

        ls.sort(key=lambda x: x[2])
        result = []
        index = -1
        for i in ls:
            if index >= i[1]:
                continue
            result.append(s[i[1]:(i[2] + 1)])
            index = i[2]
        return result


# Press the green button in the gutter to run the script.
if __name__ == '__main__':
    s = Solution()
    print(s.maxNumOfSubstrings("abab"))
