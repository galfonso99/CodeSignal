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
function solution(t: Tree<number>, s: number): boolean {
    return rec(t, 0, s)
}

const rec = (t: Tree<number> | null, cnt : number, tg: number) :boolean => {
    if (!t) {return false}
    if (!t.left && !t.right) {
        return cnt + t.value === tg
    }
    else {
        return rec(t.left, cnt+t.value, tg) || rec(t.right, cnt+t.value, tg)
    }
}