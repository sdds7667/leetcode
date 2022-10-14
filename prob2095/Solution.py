from typing import List, Optional

# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
class Solution:
    def deleteMiddle(self, head: Optional[ListNode]) -> Optional[ListNode]:
        fast = head
        count = 0
        while fast is not None:
            fast = fast.next
            count += 1

        if count == 1:
            return None
            
        if count == 2:
            head.next = None
            return head

        count //= 2
        slow = head
        for _ in range(count-1):
            slow = slow.next

        slow.next = slow.next.next
        return head

        