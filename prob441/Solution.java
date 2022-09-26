class Solution {
    public int arrangeCoins(int n) {
        return (((int) Math.sqrt(((long) n) * 8 + 1)) - 1) / 2;
    }
}
