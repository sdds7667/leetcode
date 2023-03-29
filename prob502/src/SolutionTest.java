import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class SolutionTest {
    @Test
    public void test() {
        assertEquals(10000, new Solution().findMaximizedCapital(1, 100, new int[]{10000, 5}, new int[]{0, 3}));
    }

}