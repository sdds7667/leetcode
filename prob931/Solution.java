class Solution {
    public int minFallingPathSum(int[][] matrix) {
        // dp, o(i) = min(o[i+1][j+1], o[i+1][j], o[i+1][j+1])
        // return min(o[0])
        
        int r = matrix.length;
        int c = matrix[0].length;
        var dp = new int[r][c];
        for(var j = 0; j < c; j++) 
            dp[r-1][j] = matrix[r-1][j];

        for(var i = r-2; i >= 0; i--)
            for(var j = 0; j < c; j++) 
                dp[i][j] = matrix[i][j] + Math.min((j == 0) ? Integer.MAX_VALUE : dp[i+1][j-1], Math.min((j == c - 1) ? Integer.MAX_VALUE : dp[i+1][j+1], dp[i+1][j]));

        int m = Integer.MAX_VALUE;
        for(var j = 0; j < c; j++) 
            m = Math.min(m, dp[0][j]);
        return m;

    }
}
