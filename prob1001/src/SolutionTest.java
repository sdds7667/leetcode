import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class SolutionTest {

    @Test
    public void test() {

        // [] => {} [[4,3],[3,1],[5,3],[0,5],[4,4],[3,3]] => [[1,1],[1,1]] => {{1, 1}, {1, 1}}
        assertArrayEquals(new int[]{1, 0, 1, 1, 0, 1}, new Solution().gridIllumination(6,
                new int[][]{{2, 5}, {4, 2}, {0, 3}, {0, 5}, {1, 4}, {4, 2}, {3, 3}, {1, 0}},
                new int[][]{{4, 3}, {3, 1}, {5, 3}, {0, 5}, {4, 4}, {3, 3}}));

    }

}