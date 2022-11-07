import java.util.Stack;

class Solution {
    public int maximum69Number (int num) {
        Stack<Integer> d = new Stack<>();
        
        for(; num > 0; num /= 10) 
            d.add(num % 10);
        
        var replaced = false;
        for(int i = 0, n = d.size(); i < n; i++) {
            int cd = d.pop();
            if (!replaced && cd == 6) {
                num = num * 10 + 9;
                replaced = true;
                continue;
            }
            num = num * 10 + cd;
        }
        return num;
    }
}
