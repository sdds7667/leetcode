import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.*;

class SolutionTest {

    public static Stream<Arguments> leetCode() {
        return Stream.of(
                Arguments.of("abcde", "ace", 3),
                Arguments.of("abc", "abc", 3),
                Arguments.of("abc", "def", 0)
        );
    }

    @ParameterizedTest
    @MethodSource("leetCode")
    public void test(String x, String y, int expected) {
        System.out.println("Testing");
        assertEquals(expected, new Solution().longestCommonSubsequence(x, y));
    }
}
