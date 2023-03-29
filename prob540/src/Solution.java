class Solution {
    public int singleNonDuplicate(int[] nums) {
        int l = 0;
        int r = nums.length - 1;
        if (l == r) return nums[0];

        while (l <= r) {
            var m = (l + r) / 2;
            var gsi = m;
            var gei = m;

            if (m == 0) return nums[m];
            if (m == nums.length - 1) return nums[m];

            if (nums[m - 1] == nums[m]) gsi = m - 1;
            else if (nums[m + 1] == nums[m]) gei = m + 1;
            if (gsi == gei) return nums[gsi];
            if (gsi % 2 == 0) l = gei + 1;
            else r = gsi - 1;
        }
        return nums[r];
    }
}