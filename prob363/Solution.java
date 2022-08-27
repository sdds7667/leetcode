class Solution {
    public int maxSumSubmatrix(int[][] matrix, int k) {
        int r = matrix.length, c = matrix[0].length;
        int[][] ps = new int[r+1][c+1];

        for(var i = 1; i <= r; i++) {
            int cs = 0;
            for(int j = 1; j <= c; j++) {
                cs += matrix[i-1][j-1] ;
                ps[i][j] = cs + ps[i-1][j];
                if (ps[i][j] == k) 
                    return k;
                
            }
        }

       
        int s;
        int b = Integer.MIN_VALUE;
        for(var i = 0; i < r; i++) 
            for(var j = 0; j < c; j++) 
                for(var m = i+1; m <= r; m++) 
                    for(var l = j+1; l <= c; l++) 
                        if ((s = (ps[m][l] - ps[i][l] - ps[m][j] + ps[i][j])) == k) return k;
                        else if (s < k) b = Math.max(b, s);
        return b;
    }
}