public class ListNode {
    int val;
    ListNode next;
    ListNode() {}
    ListNode(int val) { this.val = val; }
    ListNode(int val, ListNode next) { this.val = val; this.next = next; }

    boolean equals(Object other) {
        if (other == this) return true;
        if (!(other instanceof ListNode)) return false;

        return (((ListNode) other).val == this.val);
    }
}