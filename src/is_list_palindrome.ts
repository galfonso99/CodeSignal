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
function solution(head: ListNode<number>): boolean {
    let fast = head;
    let slow = head;
    
    while (fast && fast.next) {
        slow = slow.next;
        fast = fast.next.next;
    }
    
    fast = head;
    slow = reverse(slow);
    
    while (slow) {
        if (slow.value !== fast.value) {
            return false;
        }
        slow = slow.next;
        fast = fast.next;
    }
    return true;
}

let reverse = head => {
    let prevNode = null;
    
    while (head !== null) {
        let nextNode = head.next;
        head.next = prevNode;
        prevNode = head;
        head = nextNode;
    }
    
    return prevNode;
}
