class Solution {
    public int shipWithinDays(int[] weights, int days) {
//        return bruteForce(weights, 0, 0, days);

        int sum = 0, n = weights.length;
        int l = Integer.MAX_VALUE;
        int r = 0;
        for (int i : weights) {
            l = Math.min(l, i);
            r += i;
        }

        int answer = Integer.MAX_VALUE;
        while (l < r) {
            int m = (l + r) / 2;
            int days_used = 0;
            int current = 0;
            for(int i: weights) {
                current += i;
                if (current > m) {
                    days_used++;
                    current = i;
                }
            }
            if (days_used < days) {
                answer = m;
                r = m - 1;
            } else {
                l = m + 1;
            }
        }

        return answer;
    }

}
