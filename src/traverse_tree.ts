//
// Binary trees are already defined with this interface:
// class Tree<T> {
//   value: T;
//   left: Tree<T>;
//   right: Tree<T>;
//
//   constructor(value: T) {
//     this.value = value;
//     this.left = null;
//     this.right = null;
//   }
// }
function solution(t: Tree<number>): number[] {
    if (!t) {return []}
    let queue = [t]
    let arr = []
    
    while (queue.length > 0) {
        let node = queue.shift()
        arr.push(node.value)
        if (node.left) {queue.push(node.left)}
        if (node.right) {queue.push(node.right)}
    }
    return arr
}