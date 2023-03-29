// Definition for a QuadTree node.
class Node {
    public boolean val;
    public boolean isLeaf;
    public Node topLeft;
    public Node topRight;
    public Node bottomLeft;
    public Node bottomRight;


    public Node() {
        this.val = false;
        this.isLeaf = false;
        this.topLeft = null;
        this.topRight = null;
        this.bottomLeft = null;
        this.bottomRight = null;
    }

    public Node(boolean val, boolean isLeaf) {
        this.val = val;
        this.isLeaf = isLeaf;
        this.topLeft = null;
        this.topRight = null;
        this.bottomLeft = null;
        this.bottomRight = null;
    }

    public Node(boolean val, boolean isLeaf, Node topLeft, Node topRight, Node bottomLeft, Node bottomRight) {
        this.val = val;
        this.isLeaf = isLeaf;
        this.topLeft = topLeft;
        this.topRight = topRight;
        this.bottomLeft = bottomLeft;
        this.bottomRight = bottomRight;
    }
};

class Solution {
    public Node construct(int[][] grid) {
        var prefixSum = computePrefixSum(grid);
        return construct(prefixSum, 0, 0, grid.length - 1, grid[0].length - 1);
    }

    public Node construct(int[][] prefixSum, int x1, int y1, int x2, int y2) {
        var sum = sumOver(prefixSum, x1, y1, x2, y2);
        if (sum == 0)
            return new Node(false, true);
        else if (sum == (x2 + 1 - x1) * (y2 + 1 - y1))
            return new Node(true, true);


        // split in 4 regions
        int il = x1 + (x2 - x1) / 2;
        int ol = x2 - (x2 - x1) / 2;
        int it = y1 + (y2 - y1) / 2;
        int ot = y2 - (y2 - y1) / 2;
        return new Node(false, false,
                construct(prefixSum, x1, y1, il, it),
                construct(prefixSum, ol, y1, x2, it),
                construct(prefixSum, x1, ot, il, y2),
                construct(prefixSum, ol, ot, x2, y2)
        );
    }


    public boolean sameValues(int[][] prefixSum, int x1, int y1, int x2, int y2) {
        var sum = sumOver(prefixSum, x1, y1, x2, y2);
        return sum == 0 || sum == (x2 + 1 - x1) * (y2 + 1 - y1);
    }

    public int sumOver(int[][] prefixSum, int l, int t, int r, int b) {
        return prefixSum[b + 1][r + 1] - prefixSum[t][r + 1] - prefixSum[b + 1][l] + prefixSum[t][l];
    }

    public int[][] computePrefixSum(int[][] grid) {
        int[][] prefixSum = new int[grid.length + 1][grid[0].length + 1];
        for (int i = 0, n = grid.length; i < n; i++) {
            int currentRowSum = grid[i][0];
            prefixSum[i + 1][1] = currentRowSum + prefixSum[i][1];
            for (int j = 1, m = grid[0].length; j < m; j++) {
                currentRowSum += grid[i][j];
                prefixSum[i + 1][j + 1] = currentRowSum + prefixSum[i][j + 1];
            }
        }
        return prefixSum;
    }
}
