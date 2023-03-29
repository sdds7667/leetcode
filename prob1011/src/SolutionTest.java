import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class SolutionTest {
    @Test
    public void test() {
        assertEquals(4, new Solution().shipWithinDays(new int[]{3, 3, 1}, 2));
    }

}