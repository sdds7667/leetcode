
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

class Solution
{
public:
    Solution()
    {
        ios_base::sync_with_stdio(0);
        cin.tie(0);
        cout.tie(0);
    }
    vector<double> averageOfLevels(TreeNode *root)
    {
        queue<TreeNode *> *bfs = new queue<TreeNode *>;
        queue<TreeNode *> *nlv = new queue<TreeNode *>;
        bfs->push(root);
        vector<double> res;
        int s = 0;
        int c = 0;
        while (!bfs->empty())
        {
            auto current = bfs->back();
            bfs->pop();
            if (current->left != nullptr)
                nlv->push(current->left);
            if (current->right != nullptr)
                nlv->push(current->right);
            s += current->val;
            c++;
            if (bfs->empty())
            {
                delete bfs;
                bfs = nlv;
                nlv = new queue<TreeNode *>;
                res.push_back(((double) s/c));
                s = 0;
                c = 0;
            }
        }
        return res;
    }
};

int main()
{
    /*No tests provided*/
}