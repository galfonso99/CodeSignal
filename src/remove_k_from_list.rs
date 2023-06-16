// Definition for singly-linked list:
// struct List<T> {
//     value: T,
//     next: Option<Box<List<T>>>
// }
// impl<T> List<T> {
//     fn new(v : T) -> Self {
//         List { value: v, next: None }
//     }
// }
// type ListNode<T> = Option<Box<List<T>>>;
//
fn solution(mut ln: ListNode<i32>, k: i32) -> ListNode<i32> {
    if ln.is_none() {return None}
    while let Some(head) = ln {

    }
    match ln {
        Some(list) => {
           if list.value == k { 
                solution(list.next, k)
            } else {
                Some(Box::new(List { value:list.value, next: solution(list.next, k)}))                
            }
        },
        None => None,
    }
}

// For constant space, while head element matches k set head to head.next.
// Then check if head.next matches k and if so set head.next to head.next.next
// if head.next was null/None break the loop