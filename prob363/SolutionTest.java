import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.*;

class SolutionTest {

    public static Stream<Arguments> leetCode() {
        return Stream.of(
                Arguments.of(new int[][]{new int[]{1,0,1},{0,2,-3}},2,2)
        );
    }

    @ParameterizedTest
    @MethodSource("leetCode")
    public void test(int[][] n, int k, int expected) {
        assertEquals(expected, new Solution().maxSumSubmatrix(n, k));
    }
}
