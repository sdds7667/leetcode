class Solution {
    public boolean isInterleave(String s1, String s2, String s3) {
        int n = s1.length(), m = s2.length(), o = s3.length();
        if (o != (n + m)) return false;

        // count the letters
        boolean[][] dp = new boolean[n][m];
        boolean[][] vt = new boolean[n][m];
        vt[n][m] = true;
        dp[n][m] = true;
        
        // aabcc
        // dbbca
        // aadbbbaccc

        return r(s1.toCharArray(), s2.toCharArray(), s3.toCharArray(), dp, vt, 0, 0);
    }

    boolean r(char[] s1, char[] s2, char[] s3, boolean[][] dp, boolean[][] vt, int i, int j) {
        if (i < 0 || j < 0 || i == dp.length || j == dp[0].length) return false;
        if (vt[i][j]) return dp[i][j];
        vt[i][j] = true;
        return (dp[i][j] = (s1[i] == s3[i+j] && r(s1, s2, s3, dp, vt, i+1, j) || (s2[j] == s3[i+j] && r(s1, s2, s3, dp, vt, i, j + 1)))); 
    }

    /**
     * pseudo algorithm: 
     * 
     * aabcc
     * dbbca
     * aadbbcbcac
     * 
     * dp(i, j) = if (s1[i] == s3[i+j]): dp[i + 1][j] else false || s2[j] == s3[i+j] dp[i][j+1] else false
     * 
     * // base case
     * dp[n][m] = true
     * 
     * // recursive:
     * dp(0)
     * 
     */


}
