import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.*;

class SolutionTest {

    public static Stream<Arguments> leetCode() {
        return Stream.of(
                Arguments.of(new int[][]{{3, 3, 1, 1}, {2, 2, 1, 2}, {1, 1, 1, 2}}, new int[][]{{1, 1, 1, 1}, {1, 2, 2, 2}, {1, 2, 3, 3}}),
                Arguments.of(new int[][]{{11, 25, 66, 1, 69, 7}, {23, 55, 17, 45, 15, 52}, {75, 31, 36, 44, 58, 8}, {22, 27, 33, 25, 68, 4}, {84, 28, 14, 11, 5, 50}},
                        new int[][]{{5, 17, 4, 1, 52, 7}, {11, 11, 25, 45, 8, 69}, {14, 23, 25, 44, 58, 15}, {22, 27, 31, 36, 50, 66}, {84, 28, 75, 33, 55, 68}})
//                Arguments.of(new int[][]{{1, 8, 7}, {2, 8, 7}, {8, 5, 9}, {3, 7, 1}, {7, 2, 7}, {8, 4, 6}, {7, 8, 2}, {6, 1, 9}, {4, 5, 5}, {1, 5, 1}, {3, 9, 4}, {7, 8, 4}, {4, 8, 7}, {4, 5, 2}, {4, 7, 9}, {9, 4, 9}, {6, 8, 2}}, new int[][]{}
//                ),
//                Arguments.of(new int[][]{{3, 7, 6, 9, 8, 8, 2, 5, 5, 5, 9, 9, 6, 1, 9, 1, 6},
//                        {7, 5, 2, 8, 9, 9, 5, 6, 1, 9, 7, 2, 7, 3, 4, 3, 1}, {7, 6, 9, 4, 9, 7, 2, 9, 2, 8, 6, 1, 1, 6, 3, 9, 9}}, new int[][]{})
        );
    }

        @ParameterizedTest
        @MethodSource("leetCode")
        public void test ( int[][] matrix, int[][] expected){
            System.out.println("Testing");
            new Solution().diagonalSort(matrix);
            for(var i = 0 ; i < matrix.length; i++)
                assertArrayEquals(expected[i], matrix[i]);
        }
    }
