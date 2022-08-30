class Solution {
    public void rotate(int[][] matrix) {
        int n = matrix.length;
        int l = n - 1;
        for(int i = 0; i < (n/2); i++) {
            for(jnt j = i; j < l; j++) {
                int ni = n - 1 - i;
                int nj = n - 1 - j;
                int t = matrix[i][j];
                matrix[i][j] = matrix[nj][i];
                matrix[nj][i] = matrix[ni][nj];
                matrix[ni][nj] = matrix[j][ni];
                matrix[j][ni] = t;
            }
            l--;
        }
    }
}