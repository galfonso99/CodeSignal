function solution(coins: number[], q: number[]): number {
    let S = new Set<number>()
    let tmp = new Set<number>()
    for (let i = 0; i<coins.length; i++) {
        for (let j = 1; j<=q[i]; j++) {
            tmp.add(coins[i]*j)
            for (let s of S.values()) {
                tmp.add(coins[i]*j + s)
            }
        }
        for (let t of tmp.values()) {
            S.add(t)
        }
        tmp.clear()
    }
    return S.size
}
