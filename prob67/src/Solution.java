class Solution {
    public String addBinary(String a, String b) {
        int i = a.length() - 1, j = b.length() - 1, carry = 0;
        var sb = new StringBuilder(Math.max(a.length(), b.length()) + 1);
        for (; i >= 0 && j >= 0; i--, j--) {
            int l = a.charAt(i) - '0';
            int r = b.charAt(j) - '0';
            int s = (l + r + carry);
            if (s >= 2) carry = 1;
            else carry = 0;
            s = s % 2;
            sb.append(s);
        }
        for(; i >= 0; i--) {
            int l = a.charAt(i) - '0';
            int s = (l + carry);
            if (s == 2) carry = 1;
            else carry = 0;
            s %= 2;
            sb.append(s);
        }

        for(; j >= 0; j--) {
            int l = b.charAt(j) - '0';
            int s = (l + carry);
            if (s == 2) carry = 1;
            else carry = 0;
            s %= 2;
            sb.append(s);
        }

        if (carry == 1) sb.append(1);

        sb.reverse();
        return sb.toString();
    }
}