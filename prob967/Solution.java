class Solution {
    public int[] numsSameConsecDiff(int n, int k) {
        var ls = new ArrayList<Integer>();
        for(var i = 1; i < 10; i++) 
            helper(ls, i, n - 1, k, i);
        int[] res = new int[ls.size()];
        int c = 0;
        for(int i : ls) res[c++] = i;
        return res;
    }
    
    public void helper(ArrayList<Integer> res, int root, int n, int k, int last) {
        if (n == 0) {
            res.add(root);
            return;
        }
        if (last + k < 10)
            helper(res, root * 10 + last + k, n - 1, k, last + k);
        if (k != 0 && last - k >= 0)
            helper(res, root * 10 + last - k, n - 1, k, last - k);
    }
}