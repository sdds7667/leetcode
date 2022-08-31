class NumMatrix {

    int r;
    int c;
    int[][] m;

    public NumMatrix(int[][] matrix) {
        
        r = matrix.length;
        c = matrix[0].length;

        int cs;
        for(int j = 1; j < c; j++)
            matrix[0][j] += matrix[0][j-1];
        
        for(int i = 1; i < r; i++){
            cs = 0;
            for(int j = 0; j < c; j++){
                cs += matrix[i][j];
                matrix[i][j] = cs + matrix[i-1][j];
            }
        }
        
        m = matrix;
    }
    
    public int sumRegion(int row1, int col1, int row2, int col2) {
        if (row1 == 0) {
            if (col1 == 0) return m[row2][col2];
            return m[row2][col2] - m[row2][col1-1];
        } else if (col1 == 0) {
            return m[row2][col2] - m[row1 - 1][col2];
        }
        return m[row2][col2] + m[row1-1][col1-1] - m[row1-1][col2] - m[row2][col1-1];
    }
}

