import java.util.*;

class Solution {
    public List<Integer> findAnagrams(String s, String p) {
        if (s.length() < p.length()) return List.of();
        int[] counts = new int['z' + 1];
        int d = 0;
        for (int i = 0, n = p.length(); i < n; i++)
            if (counts[p.charAt(i)]-- == 0)
                d++;


        for (int i = 0, n = p.length(); i < n; i++) {
            char c = s.charAt(i);
            if (counts[c] == 0) d++;
            if (++counts[c] == 0) d--;
        }

        ArrayList<Integer> results = new ArrayList<>();
        if (d == 0) results.add(0);
        for (int j = 0, i = p.length(), n = s.length(); i < n; i++, j++) {
            var newChar = s.charAt(i);
            var oldChar = s.charAt(j);
            if (counts[oldChar] == 0) d++;
            if (--counts[oldChar] == 0) d--;
            if (counts[newChar] == 0) d++;
            if (++counts[newChar] == 0) d--;
            if (d == 0) results.add(j+1);
        }
        System.out::println("abc");
        return results;
    }
}