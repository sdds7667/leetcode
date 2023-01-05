#include <iostream>

using namespace std;

struct ListNode {
    int val;
    ListNode *next;

    ListNode() : val(0), next(nullptr) {}

    ListNode(int x) : val(x), next(nullptr) {}

    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
public:
    ListNode *oddEvenList(ListNode *head) {
        if (head == nullptr) return nullptr;
        ListNode *odd = head;
        ListNode *even = head->next;
        ListNode *evenHead = even;


        while (odd != nullptr) {
            cout << odd->val << endl;
            if (even == nullptr) {
                odd->next = evenHead;
                break;
            }

            if (even->next == nullptr) {
                odd->next = evenHead;
                break;
            }

            odd->next = even->next;
            odd = odd->next;
            even->next = odd->next;
            even = even->next;
        }
        return head;
    }
};

void print(ListNode *list) {
    if (list == nullptr) return;
    cout << list->val;
    if (list->next == nullptr)
        cout << "\n";
    else
        cout << " ";
    print(list->next);
}

int main() {
    print(Solution().oddEvenList(new ListNode(1, new ListNode(2, new ListNode(3, new ListNode(4, new ListNode(5)))))));
    return 0;
}
