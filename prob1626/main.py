import math
from typing import List


class Solution:
    def bestTeamScore(self, scores: List[int], ages: List[int]) -> int:
        joined_mapping = list(zip(ages, scores))
        joined_mapping.sort()
        best_score = [0] * len(joined_mapping)

        for ind, i in enumerate(joined_mapping):
            cb = 0
            for j in reversed(range(0, ind)):
                if joined_mapping[j][1] > i[1]:
                    continue
                best_score[ind] = best_score[j]
                break
            best_score[ind] += i[1]
        return max(best_score)


if __name__ == "__main__":
    assert Solution().bestTeamScore([4, 5, 6, 5], ages=[2, 1, 2, 1]) == 16
