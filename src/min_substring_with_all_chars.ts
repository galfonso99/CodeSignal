function solution(s: string, t: string): string {
    let M = new Map()
    let S = new Set(t)
    let arrs = []
    for (let i = 0; i< s.length; i++) {
        if (S.has(s[i])) {
            if (M.has(s[i])) {arrs.push(Array.from(M.values()))}
            M.set(s[i], i)
        }
    }
    const delta = (arr) => Math.max(...arr) - Math.min(...arr)
    arrs.push(Array.from(M.values()))
    arrs = arrs.filter((a) => a.length === t.length)
    arrs.sort((a,b) => delta(a)-delta(b))
    arrs[0].sort((a,b) => a-b)
    return s.slice(arrs[0][0], arrs[0][arrs[0].length-1]+1)
}