function solution(strs: string[], pats: string[]): boolean {
    let map1 = new Map()
    let map2 = new Map()
    for (let i = 0; i < strs.length; i++) {
        if (!map1.get(strs[i])) {
            map1.set(strs[i], pats[i])
        }
        else if (map1.get(strs[i]) !== pats[i]) {return false}
        if (!map2.get(pats[i])) {
            map2.set(pats[i], strs[i])
        }
        else if (map2.get(pats[i]) !== strs[i]) {return false}
    }
    return true
}