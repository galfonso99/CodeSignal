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
function solution(a: ListNode<number>, b: ListNode<number>): ListNode<number> {
    let x = reverse(a)
    let y = reverse(b)
    let dummy = new ListNode<number>(-1)
    let res = dummy
    let carry = 0
    let sum = 0 
    while(x && y || carry>0) {
        if (x && y) {
            sum = x.value + y.value + carry
            x = x.next
            y = y.next
        } else if (x) {
            sum = x.value + carry
            x = x.next
        } else if (y) {
            sum = y.value + carry
            y = y.next
        }
        else {break}
        carry = 0
        if (sum > 9999) {
            sum%= 10000
            carry = 1
        }
        res.next = new ListNode<number>(sum)
        res = res.next
    }
    if (!x && !y) {if (carry>0) res.next = new ListNode<number>(1)}
    else if (x) {res.next = x}
    else {res.next = y}
    console.log(dummy)
    return reverse(dummy.next)
}

let reverse = (l:ListNode<number>) :ListNode<number> => {
    let prev = null
    while(l) {
        let tmp = l.next
        l.next = prev
        prev = l
        l = tmp
    }
    return prev
}