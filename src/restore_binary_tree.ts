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
function solution(inorder: number[], preorder: number[]): Tree<number> {
    var p = 0,
        map = [];
    inorder.forEach((v, i) => map[v + 1e5] = i);
    var restore = (i, j) => {
        if (i > j) return null;
        var node = new Tree(preorder[p++]);
        if (i === j) return node;
        var m = map[node.value + 1e5];
        node.left = restore(i, m - 1);
        node.right = restore(m + 1, j);
        return node;
    };
    return restore(0, inorder.length - 1);
}