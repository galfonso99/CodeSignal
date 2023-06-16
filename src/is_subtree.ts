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
function solution(t1: Tree<number> | null, t2: Tree<number> | null): boolean {
    if (!t2) {return true}
    return rec(t1,t2)
}
const isEqual = (t1: Tree<number> | null, t2: Tree<number> | null) :boolean => {
    if (!t1 && !t2) {return true}
    if (!t1 || !t2) {return false}
    if (t1.value !== t2.value) {return false}
    return isEqual(t1.left, t2.left) && isEqual(t1.right, t2.right)
}

const rec = (t1: Tree<number> | null, t2: Tree<number> | null) :boolean => {
    if (!t1) {return false}
    if (t1.value !== t2.value) {
        return rec(t1.left,t2) || rec(t1.right,t2)
    }
    return isEqual(t1,t2) || rec(t1.left,t2) || rec(t1.right,t2)
}