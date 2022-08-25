
import java.util.*;

class Solution {
    public List<Integer> grayCode(int n) {

        // faster property:
        // for(var i = 0; i < (1 << n); i++) r.add(i ^ (i >> 1))
        
        LinkedList<Integer> r = new LinkedList<Integer>();
        recurse(n, new HashSet<>(List.of(0)), 0, r);
        return r;
    }

    public boolean recurse(int n, Set<Integer> a, int last, LinkedList<Integer> r) {
        r.addLast(last);
        if (r.size() == (1 << n)) return true;
        
        for(var i = 0; i < n; i++) {
            int j = last ^ (1 << i);
            if (!a.contains(j)) {
                a.add(j);
                if (recurse(n, a, j, r)) return true; 
                a.remove(j);
            }
        }
        r.removeLast();
        return false;
    }
}
