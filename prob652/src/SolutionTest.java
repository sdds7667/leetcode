import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class SolutionTest {
    @Test
    public void test() {
         var nodes = new TreeNode(0);
         var nodes1 = new TreeNode(0);
         var nodes2 = new TreeNode(0);
         var nodes3 = new TreeNode(0);
         var n1p = new TreeNode(0, nodes, nodes1);
         var n2p = new TreeNode(0, nodes2, nodes3);
         var n11p = new TreeNode(0, n1p, null);
         var n21p = new TreeNode(0, null, n2p);
         var root = new TreeNode(0, n11p, n21p);
         assertEquals(List.of(nodes, n1p), new Solution().findDuplicateSubtrees(root));
    }

}