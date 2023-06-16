function solution(nums: number[], m: number): number {
    let s = 0, p = 1
    for (let x of nums) {
        s = (p + s*x) % m
        p = p*x % m
    }
    return s
}