class Solution {
    public long minimumTime(int[] time, int totalTrips) {
        long min = Integer.MAX_VALUE;
        long maxValue = Integer.MIN_VALUE;
        for(int i = 0; i < time.length; i++){
            maxValue = Math.max(maxValue, time[i]);
            min = Math.min(maxValue, time[i]);
        }
        long left = min * totalTrips;
        long right;
        try{
            right = Math.multiplyExact(maxValue, totalTrips - time.length + 1);
        } catch (ArithmeticException e) {
            right = Integer.MAX_VALUE;
        }
        right = Math.max(right, Integer.MAX_VALUE);
        long ans = right;
        while (left <= right) {
            long mid = left + (right - left) / 2;
            if (check(time, totalTrips, mid)) {
                ans = mid;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        return ans;
    }

    private boolean check(int[] time, int totalTrips, long mid) {
        int count = 0;
        for (int i = 0; i < time.length; i++) {
            count += mid / time[i];
        }
        return count >= totalTrips;
    }
}