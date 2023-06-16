function solution(nums: number[]): string[] {
    let cnt = 0
    let ranges = []
    while (cnt < nums.length) {
        let i = cnt
        while (nums[cnt]+1===nums[cnt+1]) {cnt++}
        i === cnt ? ranges.push(`${nums[i]}`) : ranges.push(`${nums[i]}->${nums[cnt]}`)
        cnt++
    }
    return ranges
}