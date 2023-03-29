import java.util.Comparator;
import java.util.PriorityQueue;

class Solution {
    public int findMaximizedCapital(int k, int w, int[] profits, int[] capital) {

        var maxHeap = new PriorityQueue<Integer>(Comparator.reverseOrder());
        Quick.sort(profits, capital, 0, profits.length - 1);

        if (capital[0] > w) return 0;
        int l = 0;
        int oldl = -1;
        // each step find the most
        while (k > 0) {
            int r = profits.length - 1;
            while (l < r) {
                int m = r - (r - l) / 2;
                if (capital[m] <= w) {
                    l = m;
                } else r = m - 1;
            }
            if (oldl != l) {
                for (int i = oldl + 1; i <= l; i++)
                    maxHeap.add(profits[i]);
                oldl = l;
            }
            if (maxHeap.isEmpty()) return w;
            w += maxHeap.poll();
            k--;
        }
        return w;
    }
}

class Quick {
    public static void sort(int[] profits, int[] capital, int l, int r) {
        if (l < 0 || r < 0 || l >= r) return;
        var p = partition(profits, capital, l, r);
        sort(profits, capital, l, p);
        sort(profits, capital, p + 1, r);
    }

    public static int partition(int[] profits, int[] capital, int l, int r) {
        int pv = capital[(l + r) / 2];
        int pf = profits[(l + r) / 2];

        l -= 1;
        r += 1;
        while (true) {
            do l++; while (capital[l] < pv);
            do r--; while (capital[r] > pv);
            if (l >= r) return r;
            int t = profits[l];
            profits[l] = profits[r];
            profits[r] = t;

            t = capital[l];
            capital[l] = capital[r];
            capital[r] = t;
        }
    }
}
