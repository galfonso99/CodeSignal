function solution(nums: number[], queries: number[][]): number {
    let total = 0
    let n = Array(nums.length).fill(0)
    n[0] = nums[0]
    for (let i = 1; i< nums.length; i++) {
        n[i] = n[i-1] + nums[i]
    }
    for (let i = 0; i<queries.length; i++) {
        if (queries[i][0] === 0) {total += n[queries[i][1]]}
        else {total += n[queries[i][1]] - n[queries[i][0]-1]}
    }
    return total < 0 ? 1e9+7 - (Math.abs(total) % (1e9+7)): (total+1e9+7) % (1e9+7)
    
}