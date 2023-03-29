class Solution {
    public boolean isAlienSorted(String[] words, String order) {
        int[] ord = new int['z'+1];
        for(int i = 0; i < 26; i++)
            ord[order.charAt(i)] = i;

        for(int i = 1, n = words.length; i < n; i++)
            if (!isLessThan(ord, words[i-1], words[i]))
                return false;
        return true;
    }

    private boolean isLessThan(int[] order, String word, String word1) {
        for(int i = 0; i < word.length(); i++) {
            if (i == word1.length()) return false;
            int l = order[word.charAt(i)];
            int r = order[word1.charAt(i)];
            if (l == r) continue;
            return l < r;
        }
        return true;
    }
}