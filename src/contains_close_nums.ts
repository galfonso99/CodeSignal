function solution(nums: number[], k: number): boolean {
    let map = new Map()
    for (let i = 0; i<nums.length; i++) {
        if (Math.abs(map.get(nums[i]) - i) <= k) {
            return true
        }
        else {
            map.set(nums[i], i)
        }
    }
    return false
}