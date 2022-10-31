class Solution {
    public boolean isToeplitzMatrix(int[][] matrix) {
        int h = matrix.length, w = matrix[0].length;
        int sy = h - 1;
        int sx = 0;

        for(var d = 0; d < (w+h-1); d++) {
            int diag_length = 0;
            if (sy == 0) {
                if (sx == 0) {
                    diag_length = Math.min(w, h);
                } else {
                    diag_length = Math.min(w - sx, h);
                } 
            } else {
                diag_length = Math.min(w, h - sy);
            }
            int target = matrix[sy][sx];
            // System.out.println("diag: ("+sy+","+sx+") of length" + diag_length);
            for(int k = 1; k < diag_length; k++) {
                if (matrix[sy+k][sx+k] != target) {
                    return false;
                }
            }
            if (sy == 0)
                sx += 1;
            else 
                sy -= 1;
        }
        return true;
    }
}