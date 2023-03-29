import java.util.*;

class Solution {
    public List<Integer> addToArrayForm(int[] num, int k) {
        var result = new LinkedList<Integer>();
        int carry = 0;
        int i = num.length - 1;
        for (; k > 0 && i >= 0; k /= 10, i--) {
            int d = carry + (k % 10) + num[i];
            carry = d / 10;
            d %= 10;
            result.addFirst(d);
        }

        for(; k > 0; k /= 10) {
            int d = carry + (k % 10);
            carry = d / 10;
            d %= 10;
            result.addFirst(d);
        }

        for(; i >= 0; i--) {
            int d = carry + num[i];
            carry = d / 10;
            d %= 10;
            result.addFirst(d);
        }

        if (carry != 0) result.addFirst(carry);

        return result;
    }
}