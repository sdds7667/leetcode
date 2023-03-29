import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.Set;

class Solution {
    public int[] gridIllumination(int n, int[][] lamps, int[][] queries) {

        int[] result = new int[queries.length];
        Map<Integer, Integer> row = new HashMap<>();
        Map<Integer, Integer> col = new HashMap<>();
        Map<Integer, Integer> diag1 = new HashMap<>();
        Map<Integer, Integer> diag2 = new HashMap<>();
        Set<Integer> lampSet = new HashSet<>();

        for (int[] lamp : lamps) {
            int x = lamp[0];
            int y = lamp[1];
            if (lampSet.contains(x * n + y)) continue;
            lampSet.add(x * n + y);
            row.put(x, row.getOrDefault(x, 0) + 1);
            col.put(y, col.getOrDefault(y, 0) + 1);
            diag1.put(x - y, diag1.getOrDefault(x - y, 0) + 1);
            diag2.put(x + y, diag2.getOrDefault(x + y, 0) + 1);
        }

        int[][] dirs = {{0, 0}, {0, 1}, {0, -1}, {1, 0}, {-1, 0}, {1, 1}, {1, -1}, {-1, 1}, {-1, -1}};

        for (int i = 0; i < queries.length; i++) {
            int x = queries[i][0];
            int y = queries[i][1];
            if (row.getOrDefault(x, 0) > 0 || col.getOrDefault(y, 0) > 0 || diag1.getOrDefault(x - y, 0) > 0 || diag2.getOrDefault(x + y, 0) > 0) {
                result[i] = 1;
            }
            for (int[] dir : dirs) {
                int newX = x + dir[0];
                int newY = y + dir[1];
                if (newX < 0 || newX >= n || newY < 0 || newY >= n) continue;
                if (lampSet.contains(newX * n + newY)) {
                    lampSet.remove(newX * n + newY);
                    row.put(newX, row.getOrDefault(newX, 0) - 1);
                    col.put(newY, col.getOrDefault(newY, 0) - 1);
                    diag1.put(newX - newY, diag1.getOrDefault(newX - newY, 0) - 1);
                    diag2.put(newX + newY, diag2.getOrDefault(newX + newY, 0) - 1);
                }
            }
        }

        return result;
    }
}