class Solution {
    public String makeGood(String s) {
        int n = s.length();
        char[] cs = new char[n];
        int csl = 0;

        for(int i = 0; i < n; i++) {
            char c = s.charAt(i);
            if (csl != 0 && ((c - cs[csl-1]) == 32 || (c - cs[csl-1] == -32)))
                csl -= 1;
            else 
                cs[csl++] = c;
        }

        var sb = new StringBuilder(csl);
        for(var i = 0; i < csl; i++) 
            sb.append(cs[i]);
        
        return sb.toString();
    }
}