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
    let queue: [Tree<number>, number][] = [[t,0]]
    let arr = []
    let max = t.value
    let level = 0
    while(queue.length>0) {
        let [node,lvl] = queue.shift()
        if (lvl > level) {arr.push(max); max = node.value; level++}
        if (node.value > max) {max = node.value}
        if (node.left) {queue.push([node.left, lvl+1])}
        if (node.right) {queue.push([node.right, lvl+1])}
    }
    arr.push(max)
    return arr
}