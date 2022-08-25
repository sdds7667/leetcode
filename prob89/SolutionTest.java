import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.*;

import java.util.List;

class SolutionTest {

    public static Stream<Arguments> leetCode() {
        return Stream.of(
                Arguments.of(2, List.of(0,1,3,2)),
                Arguments.of(1, List.of(0, 1))
        );
    }

    @ParameterizedTest
    @MethodSource("leetCode")
    public void test(int n, List<Integer> expected) {
        assertEquals(expected, new Solution().grayCode(n));
    }

    public static Stream<Arguments> diff() {
        return Stream.of(
            Arguments.of(0, 1, true),
            Arguments.of(1, 0, true),
            Arguments.of(0, 2, true),
            Arguments.of(0, 3, false),
            Arguments.of(1, 3, true),
            Arguments.of(0, 8, true)
        );
    }

    @ParameterizedTest
    @MethodSource("diff")
    public void testDiff(int a, int b, boolean expected) {
        assertEquals(expected, new Solution().diffIs1(a, b));
    }
}
