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
function solution(l: ListNode<number>, k: number): ListNode<number> {
    let dummy = new ListNode<number>(-1)
    let ref = dummy
    let walk = l
    let len = 0
    while(walk) {
        len++
        walk = walk.next
    }
    let prev = null
    for (let x=0; x<Math.floor(len/k); x++) {
        for (let i=0; i<k;i++) {
            let tmp = l.next
            l.next = prev
            prev = l
            l = tmp
        }
        dummy.next = prev
        prev = null
        for (let i=0; i<k;i++) {
            dummy = dummy.next
        }
    }
    if (l) {dummy.next = l}
    return ref.next
}