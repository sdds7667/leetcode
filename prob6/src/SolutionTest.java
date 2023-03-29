import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class SolutionTest {

    @Test
    public void test(){
        assertEquals(new Solution().convert("ABCD", 2), "ACBD");

    }

}