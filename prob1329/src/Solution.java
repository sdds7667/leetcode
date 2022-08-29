class Solution {
    public int[][] diagonalSort(int[][] mat) {
        int r = mat.length;
        int c = mat[0].length;
        int dc = r + c - 1;
        for (var i = 0; i < dc; i++) {
            int sx = (i < r) ? (r - 1 - i) : 0;
            int sy = (i < r) ? 0 : i - r + 1;
            int len = (i < r) ? Math.min(i + 1, c) : Math.min(r, dc - i);
            quicksort(mat, sx, sy, 0, len - 1);
        }
        return mat;
    }

    public void quicksort(int[][] mat, int sx, int sy, int lo, int hi) {
        if (lo >= hi || lo < 0) return;
        var p = partition(mat, sx, sy, lo, hi);
        quicksort(mat, sx, sy, lo, p);
        quicksort(mat, sx, sy, p + 1, hi);
    }

    public int partition(int[][] mat, int sx, int sy, int lo, int hi) {
        var pv = mat[sx + (lo + hi) / 2][sy + (lo + hi) / 2];
        lo--;
        hi++;
        while (true) {
            do lo++;while (mat[sx + lo][sy + lo] < pv);
            do hi--;while (mat[sx + hi][sy + hi] > pv);
            if (lo >= hi) return hi;
            int t = mat[sx + lo][sy + lo];
            mat[sx + lo][sy + lo] = mat[sx + hi][sy + hi];
            mat[sx + hi][sy + hi] = t;
        }
    }
}