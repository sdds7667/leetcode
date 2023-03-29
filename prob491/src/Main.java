import java.util.*;

class Solution {
    public int findLongestChain(int[][] pairs) {
        Arrays.sort(pairs, Comparator.comparingInt(l -> l[1]));
        int l = 1;
        int t = pairs[0][1];
        for(int i = 1, n = pairs.length; i < n; i++)
            if (pairs[i][0] > t) {
                l++;
                t = pairs[i][1];
            }
        return l;
    }
}

public class Main {
    public static void main(String[] args) {
        for (var i : new Solution().findSubsequences(new int[]{4, 5, 4, 5})) {
            for (int j : i) {
                System.out.print(j + " ");
            }
            System.out.println();
        }
    }
}
