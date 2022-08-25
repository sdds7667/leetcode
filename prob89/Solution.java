
import java.util.*;

class Solution {
    public List<Integer> grayCode(int n) {
        LinkedList<Integer> r = new LinkedList<Integer>();
        recurse(n, new HashSet<>(), 0, r);
        return r;
    }

    public boolean recurse(int n, Set<Integer> a, int last, LinkedList<Integer> r) {
        r.addLast(last);
        if (r.size() == (1 << n)) return true;
        
        for(var i = 0; i < n; i++) {
            int j = last & (1 << i);
            if (!a.contains(j)) {
                if (diffIs1(j, last)) {
                    a.add(i);
                    if (recurse(n, a, j, r)) return true; 
                    a.remove(i);
                }
            }
        }
        r.removeLast();
        return false;
    }

    public boolean diffIs1(int a, int b) {
        return ((65536 % (a ^ b)) == 0);
    }
}