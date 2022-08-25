class Solution {
    public ListNode reverseBetween(ListNode head, int left, int right) {
        if (left == right) return head;
        if (left == 1) {
            ListNode tail = head;
            for(var i = 1; i < right; i++) 
                tail = tail.next;

            ListNode ref = tail.next;
            tail.next = null;

            reverseList(head);
            head.next = ref;
            return tail;
        } 
        ListNode ih = head;
        ListNode tl = head;
        for(var i = 1; i < right; i++) {
            if (i < left) ih = ih.next;
            tail = tail.next;
        }
        ListNode r = tail.next;
        tail.next = null;
        reverseList(ih.next);
        ih.next.next = ref;
        ih.next = tail;
        return head;
    }

    public ListNode reverseList(ListNode head) {
        ListNode prev = null;
        ListNode i = head;
        ListNode j = head.next;
        while( i != null) {
            i.next = prev;
            prev = i;
            i = j;
            if (j != null) j = j.next;
        }
        return i;
    }
}