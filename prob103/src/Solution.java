import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.List;

class TreeNode {
    int val;
    TreeNode left;
    TreeNode right;

    TreeNode() {
    }

    TreeNode(int val) {
        this.val = val;
    }

    TreeNode(int val, TreeNode left, TreeNode right) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
}

class Solution {
    public List<List<Integer>> zigzagLevelOrder(TreeNode root) {
        var result = new ArrayList<List<Integer>>();
        if (root == null) return result;
        var queue = new ArrayList<TreeNode>();
        var left = true;
        queue.add(root);
        while (queue.size() > 0) {
            var newQueue = new ArrayList<TreeNode>();

            // method: add all the elements from the queue to the results list

            var resultingArrayList = new ArrayList<Integer>();
            if (left)
                for (var node : queue)
                    resultingArrayList.add(node.val);
            else
                for (var i = queue.size() - 1; i >= 0; i--)
                    resultingArrayList.add(queue.get(i).val);
            result.add(resultingArrayList);

            left = !left;

            for(var node: queue) {
                if (node.left != null) newQueue.add(node.left);
                if (node.right != null) newQueue.add(node.right);
            }
            queue = newQueue;
        }

        return result;
    }
}