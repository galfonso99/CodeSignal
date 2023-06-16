#![allow(clippy::needless_return)]


mod palindrome;
mod adjacent_product;
mod shape_area;

struct List<T> {
    value: T,
    next: Option<Box<List<T>>>
}
impl<T> List<T> {
    fn new(v : T) -> Self {
        List { value: v, next: None }
    }
}
type ListNode<T> = Option<Box<List<T>>>;


fn main() {
    //palindrome::run();
    //adjacent_product::run();
    //shape_area::run();
}

fn solution(mut ln: ListNode<i32>, k: i32) -> ListNode<i32> {
    if ln.is_none() {return None}
    let head = &ln;
    let list = &ln;
    while let Some(head) = ln {
        if head.value == k {
            ln = head.next;
        } else {break;}
    }
    *head.clone()
}
