
#include<bits/stdc++.h>

using namespace std;


class Node {
public:
    int val;
    vector<Node*> children;

    Node() {}

    Node(int _val) {
        val = _val;
    }

    Node(int _val, vector<Node*> _children) {
        val = _val;
        children = _children;
    }
};


class Solution {
public:

    vector<int> res;
    Solution() {
        ios_base::sync_with_stdio(0);
        cin.tie(0);
        cout.tie(0);
    }

    vector<int> preorder(Node* root) {
        if (root == nullptr) return;
        res.push_back(root->val);
        for(auto* r: root->children)
            preorder(r);
        return res;
    }
    
};

int main() {}