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
// }=
 
function solution(t, k) {
    var i = 0,
        parent = {},
        node = t;
    while (true) {
        if (node.left) { // enter left child
            var next = node.left;
            node.left = parent;
            parent = node;
            node = next;
        } else { // visit leftmost node
            if (++i === k) return node.value;
            var rightChild = node.right;
            node.right = parent;
            if (rightChild) { // enter right child
                parent = node;
                node = rightChild;
            } else { // backtrack to nearest unvisited parent
                while (!node.left) node = node.right;
                parent = node.left;
                node.left = null;
            }
        }
    }
}