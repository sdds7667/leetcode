import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.*;

class SolutionTest {

    public static Stream<Arguments> leetCode() {
        return Stream.of(
                Arguments.of("25525511135", List.of("255.255.11.135","255.255.111.35")),
                Arguments.of("0000", List.of("0.0.0.0"))
        );
    }

    @ParameterizedTest
    @MethodSource("leetCode")
    public void test(int x, int y, int expected) {
        System.out.println("Testing");
        assertEquals(expected, new Solution().add(x, y));
    }
}
