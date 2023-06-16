function solution(a: number[], b: number[], v: number): boolean {
    let S = new Set()
    for (let x of a) {
        S.add(v - x)
    }
    for (let x of b) {
        if (S.has(x)) {return true}
    }
    return false
}