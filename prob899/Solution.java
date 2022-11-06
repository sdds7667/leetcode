import java.nio.charset.CharsetEncoder;
import java.util.Arrays;

class Solution {
    public String orderlyQueue(String s, int k) {
        var chars = s.toCharArray();
        if (k >= 2) {
            Arrays.sort(chars);
            return new String(chars);
        }

        int bestIndex = 0;

        for(int i = 1, n = chars.length; i < n; i++) {
            int l = bestIndex;
            int r = i;
            for(int j = 0; j < n; j++, r = (r + 1) % n, l = (l+1) % n) {
                if (chars[l] == chars[r]) continue;
                if (chars[l] > chars[r]) 
                    bestIndex = r;
                break;
            }
        }

        var sb = new StringBuilder(chars.length);
        for (int i = 0, n = chars.length, r = bestIndex; i < n; i++, r = (r+1) % n) {
            sb.append(chars[r]);
        }
        return sb.toString();
    }
}
