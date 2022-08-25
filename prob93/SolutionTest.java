import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.*;

class SolutionTest {

    public static Stream<Arguments> leetCode() {
        return Stream.of(
                Arguments.of(1, 2, 3),
                Arguments.of(-1, 1, 0)
        );
    }

    @ParameterizedTest
    @MethodSource("leetCode")
    public void test(int x, int y, int expected) {
        System.out.println("Testing");
        assertEquals(expected, new Solution().add(x, y));
    }
}
