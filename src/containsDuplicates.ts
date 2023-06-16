function solution(a: number[]): boolean{
    let s = new Set();
    for (let e of a) {
        if (s.has(e)) {return true}
        else {
            s.add(e)
        }
    }
    return false
}