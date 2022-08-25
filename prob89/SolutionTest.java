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

}
