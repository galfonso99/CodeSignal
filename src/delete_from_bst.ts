class Tree<T> {
    value: T;
    left: Tree<T> | null;
    right: Tree<T> | null;
  
    constructor(value: T) {
      this.value = value;
      this.left = null;
      this.right = null;
    }
  }
  function solution(t: Tree<number>, queries: number[]): Tree<number> | null {
      for (let val of queries) {
          if (!t) {return t}
          if (t.value === val) {
              if (t.left) {mutate(t)}
              else if (t.right) {
                 t = t.right 
              }  
              else {return null}
          }
          else {
              let parent = find(t, val)
              if (!parent) {continue}
              let node = parent.left && parent.left.value === val ? parent.left : parent.right
              if (node!.left) {mutate(node!)}
              else {
                  if (parent.left && parent.left.value === val) {parent.left = node!.right}
                  else {parent.right = node!.right}
              }  
          }
      }
      return t
      
  }
  
  const find = (t:Tree<number>, val:number) :Tree<number> | null => {
      if (!t) {return null}
      else if ((t.left && t.left.value === val) || (t.right && t.right.value === val)) {return t}
      else if (t.value < val) {return find(t.right!, val)}
      return find(t.left!, val)
  }
  
  
  const mutate = (t: Tree<number>) :void => {
      const _mutate = (t: Tree<number>) :number => {
          if (!t.right!.right) {let tmp = t.right!.value; t.right = t.right!.left; return tmp;}
          return _mutate(t.right!)
      }
      if (!t.left!.right) {
          let tmp = t.left!.value
          t.left = t.left!.left
          t.value = tmp
          return
      }
      t.value = _mutate(t.left!)
  }