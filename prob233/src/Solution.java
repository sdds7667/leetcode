import java.util.HashMap;

class Solution {

    HashMap<Long, Long> m = new HashMap<>();

    public int countDigitOne(int n) {
        long i = 1;
        while(i <= n) i *= 10;
        return (int) computeUpTo(n, i/10);
    }

    public long computeUpTo(long n, long p) {

        if (n == 0) return 0;
        if (n < 10) return 1;
        if (m.containsKey(n)) return m.get(n);
        long a = n / p;
        long res = computeUpTo(p-1, p/10) * a + computeUpTo(n % p, p / 10);
        if (a == 1) res += (n%p)+1;
        else res += p;
        m.put(n, res);
        return res;
    }
}