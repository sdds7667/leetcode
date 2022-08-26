import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.*;
import java.util.*;

class SolutionTest {

    public static Stream<Arguments> leetCode() {
        return Stream.of(
                Arguments.of("25525511135", List.of("255.255.11.135","255.255.111.35")),
                Arguments.of("0000", List.of("0.0.0.0")),
                Arguments.of("25525511335", List.of("255.255.113.35")),
                Arguments.of("255514342", List.of("25.55.143.42","255.5.143.42","255.51.43.42")),
                Arguments.of("25514342", List.of("2.55.143.42","25.5.143.42","25.51.43.42","255.1.43.42","255.14.3.42","255.14.34.2","255.143.4.2"))
        );
    }

    @ParameterizedTest
    @MethodSource("leetCode")
    public void test(String s, List<String> expected) {
        assertEquals(expected, new Solution().restoreIpAddresses(s));
    }
}
