class Solution {
    public String convert(String s, int numRows) {
        StringBuilder[] stringBuilders = new StringBuilder[numRows];
        for(int i = 0; i < numRows; i++)
            stringBuilders[i] = new StringBuilder();

        int row = 0;
        int direction = 1;
        for (int i = 0, n = s.length(); i < n; i++) {
            stringBuilders[row].append(s.charAt(i));
            row += direction;
            if (row == numRows) {
                direction = -1;
                row = Math.max(numRows - 2, 0);
            } else if (row == -1) {
                row = Math.min(numRows - 1, 1);
                direction = 1;
            }
        }
        StringBuilder result = new StringBuilder(s.length());
        for (var builder : stringBuilders)
            result.append(builder);
        return result.toString();
    }
}