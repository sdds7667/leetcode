import java.util.Comparator;
import java.util.HashSet;
import java.util.Objects;
import java.util.PriorityQueue;


class Solution {
    public boolean isReachable(int targetX, int targetY) {

        while (targetX != targetY) {
            if (targetX > targetY) {
                targetX -= targetY;
            } else {
                targetY -= targetX;
            }
        }
        int count = 0;
        while (targetX > 0) {
            count += (targetX & 1);
            if (count > 1) return false;
            targetX >>= 1;
        }
        return true;
    }
}