class Solution {
    public int totalFruit(int[] fruits) {
        int n = fruits.length;
        if (n == 1 || n == 2) return n;

        int best = Integer.MIN_VALUE;
        int currentSequenceLength = 1;
        int currentSequenceCoType = 0;
        int currentIdenticalSequence = 1;
        for (int i = 1; i < n; i++) {
            if (fruits[i] == fruits[i - 1]) {
                currentIdenticalSequence++;
                currentSequenceLength++;
            } else {
                if (fruits[i] == currentSequenceCoType) {
                    currentSequenceLength++;
                } else {
                    currentSequenceLength = currentIdenticalSequence + 1;
                }
                currentSequenceCoType = fruits[i - 1];
                currentIdenticalSequence = 1;
            }
            best = Math.max(best, currentSequenceLength);
        }
        return best;
    }
}
