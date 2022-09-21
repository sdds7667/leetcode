
#include<bits/stdc++.h>

using namespace std;

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */

#define v vector

class Solution {
public:
    Solution() {
        ios_base::sync_with_stdio(0);
        cin.tie(0);
        cout.tie(0);
    }
    vector<vector<vector<TreeNode*>*>> cache;
    vector<TreeNode*>* tn;

    vector<TreeNode*> generateTrees(int n) {
        tn = new vector<TreeNode*>{nullptr};
        vector<vector<TreeNode*>*> c;
        for(int j = 0; j <= n+1; j++) c.push_back(nullptr);
        for(int i = 0; i <= n+1; i++) cache.push_back(c);
        buildCache(1, n);
        return *cache[1][n];
    }

    void buildCache(int start, int end) {
        // cout << "building cache for " << start << " " << end << "\n";
        if (cache[start][end] != nullptr) return;
        if (start > end) {
            cache[start][end] = tn;
            return;
        }

        vector<TreeNode*>* res = new vector<TreeNode*>();
        for(int i = start; i <= end; i++) {
            buildCache(start, i - 1);
            buildCache(i + 1, end);
            for(const auto l: *cache[start][i-1]) 
                for(const auto r: *cache[i+1][end]) 
                    (*res).push_back(new TreeNode(i, l, r));
        }
        cache[start][end] = res;
    }
};

// bool test(UNDEFINED values, int expected) {
//     auto result = (new Solution())->;
//     if (result != expected) {
//         cout << "Got " << result << " expected " << expected << "\n";
//         cout << "Failed test\n";
//         for(const auto i: values)
//             cout << i << " ";
//         cout << "\n";
//         return false;
//     }
//     return true;
// }

// int main() {
//     cout << "Running" << "\n";
//     vector<pair<vector<int> *, int>> tests {
//     };

//     bool failed = false;

//     for(const auto& p : tests) {
//         if (!test((*p.first), p.second)) {
//             failed = true;
//             break;
//         }
//     }

//     if (!failed) 
//         cout << "Passed all tests\n";

//     cout << "Done running.\n";
//     return 0;
// }