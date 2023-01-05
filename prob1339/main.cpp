#include <iostream>

using namespace std;

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;

    TreeNode() : val(0), left(nullptr), right(nullptr) {}

    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}

    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class NodeWithSum {
    int m_ancestor_sum;
    int m_left_sum;
    int m_right_sum;
    int m_val;
    NodeWithSum *m_left;
    NodeWithSum *m_right;

public:
    NodeWithSum(NodeWithSum *left, NodeWithSum *right, int val) :
            m_left(left),
            m_right(right),
            m_ancestor_sum(0),
            m_left_sum(0),
            m_right_sum(0),
            m_val(val) {};

    int compute_sum() {
        m_left_sum = (m_left == nullptr) ? 0 : m_left->compute_sum();
        m_right_sum = (m_right == nullptr) ? 0 : m_right->compute_sum();
        return m_val + m_left_sum + m_right_sum;
    }

    int populate_ancestral_sum(int ancestral_sum) {
        m_ancestor_sum = ancestral_sum;
        if (m_left != nullptr)
            m_left->populate_ancestral_sum(ancestral_sum + m_val + m_right_sum);
        if (m_right != nullptr)
            m_right->populate_ancestral_sum(ancestral_sum + m_val + m_left_sum);
    }


    long compute_max_product() {
        long c_max = LONG_MIN;
        if (m_left != nullptr)
            c_max = max(c_max, m_left->compute_max_product());
        if (m_right != nullptr)
            c_max = max(c_max, m_right->compute_max_product());
        c_max = max(c_max, ((long) m_left_sum) * (m_ancestor_sum + m_right_sum + m_val));
        c_max = max(c_max, ((long) m_right_sum) * (m_ancestor_sum + m_left_sum + m_val));
        return c_max;
    }

};

class Solution {
public:
    int maxProduct(TreeNode *root) {

        auto *sum_root = buildSumTree(root);
        sum_root->compute_sum();
        sum_root->populate_ancestral_sum(0);
        return (int) (sum_root->compute_max_product() % (1'000'000'000 + 7));
    }

    NodeWithSum *buildSumTree(TreeNode *root) {
        if (root == nullptr) return nullptr;
        return new NodeWithSum(buildSumTree(root->left), buildSumTree(root->right), root->val);
    }

};

int main() {
    cout << Solution().maxProduct(
            new TreeNode(5, new TreeNode(6),
                         new TreeNode(6, new TreeNode(8, new TreeNode(10), nullptr),
                                      new TreeNode(6, new TreeNode(5), nullptr)))) << "\n";
    return 0;
}
