public class ListNode {
    int val;
    ListNode next;
    ListNode() {}
    ListNode(int val) { this.val = val; }
    ListNode(int val, ListNode next) { this.val = val; this.next = next; }
}

class Solution {
    public ListNode removeNthFromEnd(ListNode head, int n) {
        if (head.next == null) return null;
        var fast = head;
        for(; n>0;n--) fast = fast.next;
        var slow = head;
        if (fast == null) return head.next;
        for(;fast.next != null; fast = fast.next, slow = slow.next);
        if (slow.next.next == null) 
            slow.next = null;
        else
            slow.next = slow.next.next;
        return head;
    }
}