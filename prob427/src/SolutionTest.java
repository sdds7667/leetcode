import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class SolutionTest {
    @Test
    public void test() {
        var res = new int[][]{{0,1},{1,0}};
        var prefixSum = new Solution().computePrefixSum(res);

        assertArrayEquals(prefixSum[0], new int[]{0, 0, 0});
        assertArrayEquals(prefixSum[1], new int[]{0, 0, 1});
        assertArrayEquals(prefixSum[2], new int[]{0, 1, 2});

        assertEquals(0, new Solution().sumOver(prefixSum, 1, 1, 1, 1));
        assertEquals(1, new Solution().sumOver(prefixSum, 1, 0, 1, 0));
        assertEquals(1, new Solution().sumOver(prefixSum, 1, 0, 1, 1));
        assertEquals(2, new Solution().sumOver(prefixSum, 0, 0, 1, 1));

    }

}