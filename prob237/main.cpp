
#include<bits/stdc++.h>

using namespace std;

struct ListNode {
    ListNode* next;
    int val;
};

class Solution {
public:
    Solution()
    {
    ios_base::sync_with_stdio(0);
    cin.tie(0);
    //cout.tie(0);
    }
    void deleteNode(ListNode* node) {
        node->val = node->next->val;
        node->next = node->next->next;
    }
};

int main() {

}