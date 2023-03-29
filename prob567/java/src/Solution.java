class Solution {
    public boolean checkInclusion(String s1, String s2) {
        // keep track of the sum of differences
        if (s1.length() > s2.length()) return false;
        int[] counts = new int[125];

        int diffThenZero = 0;

        for (int i = 0; i < s1.length(); i++) {
            char c = s1.charAt(i);
            if (counts[c] == 0)
                diffThenZero++;
            counts[c]--;
        }

        for (int i = 0; i < s1.length(); i++) {
            char c = s2.charAt(i);
            if (counts[c] == 0)
                diffThenZero++;
            counts[c]++;
            if (counts[c] == 0)
                diffThenZero--;
        }
        if (diffThenZero == 0) return true;
        int j = 0;
        for(int i = s1.length(); i < s2.length(); i++) {
            char newChar = s2.charAt(i);
            char oldChar = s2.charAt(j++);
            if (counts[newChar] == 0) diffThenZero++;
            counts[newChar]++;
            if (counts[newChar] == 0) diffThenZero--;

            if (counts[oldChar] == 0) diffThenZero++;
            counts[oldChar]--;
            if (counts[oldChar] == 0) diffThenZero--;

            if (diffThenZero == 0) return true;
        }

        return false;
    }
}