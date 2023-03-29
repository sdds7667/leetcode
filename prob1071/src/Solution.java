class Solution {
    public String gcdOfStrings(String str1, String str2) {

        String smallerString = str1;
        String longerString = str2;
        if (smallerString.length() > longerString.length()) {
            smallerString = str2;
            longerString = str1;
        }

        char[] a = smallerString.toCharArray();
        char[] b = longerString.toCharArray();

        int sumA = 0;
        int sumB = 0;
        for (char c : a) sumA += c;
        for (char c : b) sumB += c;
        int smallestLength = 0;
        int sumSubstring = sumA;
        for (int i = smallerString.length() - 1; i >= 0; i--) {
            if (sumA % sumSubstring == 0 && sumB % sumSubstring == 0) {
                // check if it is a substring. for now, return true;
                return new String(a, 0, i+1);
            }
            sumSubstring -= a[i];
        }
        return "";
    }
}