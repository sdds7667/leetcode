
#include <bits/stdc++.h>

using namespace std;

struct TreeNode
{
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

bool sortComparator(const pair<int, TreeNode*>& l, const pair<int, TreeNode*>& r) {
    if (l.first == r.first) return l.second->val < r.second->val;
    return l.first < r.first;
}

class Solution
{
public:
    Solution()
    {
        ios_base::sync_with_stdio(0);
        cin.tie(0);
        cout.tie(0);
    }

    vector<vector<int>> verticalTraversal(TreeNode *root)
    {
        int minCol = INT_MAX, maxCol = INT_MIN;
        unordered_map<int, vector<pair<int, TreeNode *>>> m;
        dfs(root, 0, 0, m, &minCol, &maxCol);
        vector<vector<int>> res;

        for(auto i = minCol; i < maxCol; i++) {
            auto f = m.find(i);
            if (f == m.end()) continue;
            auto& ls = f->second;
            sort(ls.begin(), ls.end(), sortComparator);
            vector<int> crs;
            for(auto& j: ls) 
                crs.push_back(j.second->val);
            res.push_back(crs);
        }
        return res;
    }

    void dfs(TreeNode *node, int row, int col, unordered_map<int, vector<pair<int, TreeNode *>>> &m, int* minCol, int* maxCol)
    {
        if (node == nullptr)
            return;
        (*minCol) = min(*minCol, col);
        (*maxCol) = max(*maxCol, col);
        auto me = m.find(col);
        if (me == m.end())
            m.emplace(col, make_pair(row, node));
        else
            me->second.emplace_back(row, node);
        dfs(node->left, row + 1, col - 1, m, minCol, maxCol);
        dfs(node->right, row + 1, col + 1, m, minCol, maxCol);
    }
};
int main()
{
    /* Tests not provided */
}
