import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class SolutionTest {

    @Test
    public void test() {
        assertEquals("100", new Solution().addBinary("1010", "1011"));
    }

}