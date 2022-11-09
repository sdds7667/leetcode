import java.util.Stack;

class StockSpanner {
    Stack<Integer> xs;
    Stack<Integer> ys;

    public StockSpanner() {
        xs = new Stack<>();
        ys = new Stack<>();
    }
    
    public int next(int price) {
        int t = 1;
        while (!xs.isEmpty() && xs.peek() <= price) {
            t += ys.pop();
            xs.pop();
        }
        xs.push(price);
        ys.push(t);
        return t;
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * StockSpanner obj = new StockSpanner();
 * int param_1 = obj.next(price);
 */