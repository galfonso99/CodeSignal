function solution(nums: number[]): number {
    if (nums.length < 1) {return 0}
    let arr = [nums[0], Math.max(nums[0], nums[1])]
    for (let i = 2; i<nums.length+2; i++) {
        arr.push( Math.max(arr[i-2]+nums[i],arr[i-1]) )
    }
    return arr[nums.length-1]
}