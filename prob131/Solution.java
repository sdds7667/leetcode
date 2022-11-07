import java.util.*;

class Solution {
    public List<List<String>> partition(String s) {
        // brute force solution: generate all possible partitions of the string. 
        // if all partitions are a palindrome, add the partition list to the list


        // to cache the results of the partitions:
        // build a 2d table of size n := s.len(); nxn
        // p[i][j] = null if not a palindrome (starting at i, and ending at j (inclusive))
        // p[i][j] = s.substr(i, j) if palindrome

        int n = s.length();
        char[] sc = s.toCharArray();
        String[][] partitions = new String[n][n];

        for(int i = 0; i < n; i++) 
            partitions[i][i] = s.substring(i, i+1);
        
        for(int i = 0; i < n-1; i++) 
            if (sc[i] == sc[i+1])
                partitions[i][i+1] = s.substring(i, i+2);
        
        for(int i = 2; i < n; i++) 
            for(int j = 0; j < n-i; j++) 
                if (sc[j] == sc[j+i] && partitions[j+1][j+i-1] != null)
                    partitions[j][j+i] = s.substring(j, j+i+1);
        

        List<List<String>> res = new ArrayList<List<String>>();
        partition(partitions, 0, res, new LinkedList<String>());
        return res;
    }

    void partition(String[][] partitions, int i, List<List<String>> res, LinkedList<String> current) {
        if (i == partitions.length) {
            res.add(new ArrayList(current));
            return;
        }
        for(int k = i; k < partitions.length; k++) 
            if (partitions[i][k] != null) {
                current.addLast(partitions[i][k]);
                partition(partitions, k+1, res, current);
                current.pollLast();
            }
    }

    
}
