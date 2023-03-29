import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class SolutionTest {

    @Test
    public void test() {
        assertEquals(6, new Solution().compress(new char[]{'a','a','b','b','c','c','c'}));
    }


}