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
function solution(l: ListNode<number>, n: number): ListNode<number> {
    let walk  = l
    let dummy = new ListNode<number>(-1)
    let ref = dummy
    let start = l
    let len = 0
    while (walk) {len++; walk=walk.next}
    for (let i = 0; i<len-n; i++) {l = l.next}
    dummy.next = l
    while (dummy.next) {
        dummy = dummy.next
    }
    dummy.next = start 
    for (let i = 0; i<len-n; i++) {dummy = dummy.next}
    dummy.next = null
    return ref.next
    
}