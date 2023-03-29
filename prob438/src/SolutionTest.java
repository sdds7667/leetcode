import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class SolutionTest {

    @Test
    public void test() {
        assertEquals(List.of(0, 6), new Solution().findAnagrams("cbaebabacd", "abc"));
    }

}