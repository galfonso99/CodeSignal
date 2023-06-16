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
function solution(t: Tree<number>): number {
    let queue: [Tree<number>, number[]][] = [[t,[]]]
    let paths: number[][] = []
    while(queue.length>0) {
        let [node,arr] = queue.shift()
        if (node.left) {queue.push([node.left, [...arr, node.value]])}
        if (node.right) {queue.push([node.right, [...arr, node.value]])}
        if (!node.right && !node.left) {arr.push(node.value); paths.push(arr) }
    }
    return paths.map( (val, ind) => num(val)).reduce( (prev,cur) => prev+cur)
}

const num = (arr: number[]) :number => {
    let count = 0
    for (let i = 0; i<arr.length; i++) {
        count+= Math.pow(10,arr.length-i-1) * arr[i]
    }
    return count
}