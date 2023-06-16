// Singly-linked lists are already defined with this interface:
// class ListNode<T> {
//   value: T;
//   next: ListNode<T>;
//
//   constructor(value: T) {
//     this.value = value;
//     this.next = null;
//   }
// }
//
function solution(l1: ListNode<number>, l2: ListNode<number>): ListNode<number> {
    let dummy = new ListNode<number>(-1)
    let ref = dummy
    while (l1 && l2) {
        if (l1.value < l2.value) {
            dummy.next = new ListNode<number>(l1.value)
            l1 = l1.next
        }
        else {
            dummy.next = new ListNode<number>(l2.value)
            l2 = l2.next
        }
        dummy = dummy.next
    }
    if (!l1 && !l2) {return ref.next}
    else if (l1) {dummy.next = l1}
    else {dummy.next = l2}
    return ref.next
}