import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class SolutionTest {

    @Test
    public void test() {
        assertEquals(List.of(1, 2, 3, 4), new Solution().addToArrayForm(new int[]{1, 2, 0, 0}, 34));
    }

    @Test
    public void test1() {
        assertEquals(List.of(4, 5, 5), new Solution().addToArrayForm(new int[]{2, 7, 4}, 181));
    }

    @Test
    public void test2() {
        assertEquals(List.of(1, 0, 2, 1), new Solution().addToArrayForm(new int[]{2, 1, 5}, 806));
    }

}