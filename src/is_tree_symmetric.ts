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
function solution(t: Tree<number>): boolean {
    return rec(t,t)
}

const rec = (t1:Tree<number> | null ,t2: Tree<number> | null) :boolean => {
    if (!t1 && !t2) {return true}
    else if (!t1 || !t2 || t1.value !== t2.value) {return false}
    else {
        return rec(t1.left,t2.right) && rec(t1.right,t2.left)
    }
}