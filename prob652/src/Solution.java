import com.sun.source.tree.Tree;

import java.util.*;

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

class UnionFind<A> {
    HashMap<A, A> parents;
    HashMap<A, Integer> rank;

    public UnionFind() {
        parents = new HashMap<>();
        rank = new HashMap<>();
    }

    public void addNewNode(A node) {
        parents.put(node, node);
        rank.put(node, 0);
    }

    public A find(A node) {
        if (node == null) return null;
        var cp = parents.get(node);
        if (cp == node)
            return node;

        var parent = find(cp);
        parents.put(node, parent);
        return parent;
    }

    public boolean merge(A a, A b) {
        var pa = find(a);
        var pb = find(b);
        if (pa == pb) return false;
        var ar = rank.get(pa);
        var br = rank.get(pb);
        if (ar > br) {
            parents.put(pb, pa);
        } else if (br > ar) {
            parents.put(pa, pb);
        } else {
            parents.put(pb, pa);
            rank.put(pa, ar + 1);
        }
        return true;
    }

    public List<A> getClusters() {
        var result = new ArrayList<A>();
        for (var i : rank.entrySet()) {
            if (i.getValue() > 0) {
                result.add(i.getKey());
            }
        }
        return result;
    }
}

class Solution {
    public List<TreeNode> findDuplicateSubtrees(TreeNode root) {
        HashMap<TreeNode, TreeNode> parents = new HashMap<>();
        HashMap<Integer, HashSet<TreeNode>> queue = new HashMap<>();
        UnionFind<TreeNode> uf = new UnionFind<>();
        ArrayList<TreeNode> bfsQueue = new ArrayList<>();
        bfsQueue.add(root);
        while (!bfsQueue.isEmpty()) {
            var newBfsQueue = new ArrayList<TreeNode>();
            for (var i : bfsQueue) {
                uf.addNewNode(i);
                if (i.left == null && i.right == null) {
                    if (queue.containsKey(i.val)) {
                        queue.get(i.val).add(i);
                    } else {
                        queue.put(i.val, new HashSet<>(List.of(i)));
                    }
                } else {
                    if (i.left != null) {
                        newBfsQueue.add(i.left);
                        parents.put(i.left, i);
                    }

                    if (i.right != null) {
                        newBfsQueue.add(i.right);
                        parents.put(i.right, i);
                    }
                }
            }
            bfsQueue = newBfsQueue;
        }

        while (queue.size() > 0) {
            var newQueue = new HashMap<Integer, HashSet<TreeNode>>();
            for (var k : queue.entrySet()) {
                if (k.getValue().size() == 1) continue;
                var ls = new ArrayList<>(k.getValue());
                for (int i = 0, n = ls.size(); i < n; i++) {
                    var a = ls.get(i);
                    for (int j = i + 1; j < n; j++) {
                        var b = ls.get(j);
                        if (uf.find(a.left) == uf.find(b.left) && uf.find(a.right) == uf.find(b.right)) {
                            uf.merge(a, b);
                            var ap = parents.getOrDefault(a, null);
                            var bp = parents.getOrDefault(b, null);
                            if (ap == null || bp == null) continue;
                            if (ap.val == bp.val) {
                                if (newQueue.containsKey(ap.val)) {
                                    newQueue.get(ap.val).addAll(List.of(ap, bp));
                                } else {
                                    newQueue.put(ap.val, new HashSet<>(List.of(ap, bp)));
                                }

                            }
                        }
                    }
                }
            }
            queue = newQueue;
        }
        return uf.getClusters();
    }
}