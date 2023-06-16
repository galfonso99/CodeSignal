function solution(s: number, arr: number[]): number[] {
    for (let i = 1; i < arr.length; i++) {
        arr[i] += arr[i-1] 
    }
    let ind = 0, longest = [-1,-2]
    while (arr[ind] < s) { ind++ }
    for (let i = 0, j = ind; j < arr.length;) {
        let offset = arr[i-1] ? arr[i-1] : 0
        if (arr[j] - offset > s) { i++ }
        else if (arr[j] - offset < s) { j++ }
        else {
            if (longest[1] - longest[0] < j - i) {longest = [i,j]}
            j++
        }
    }
    return longest[0] === -1 ? [-1] : [longest[0]+1, longest[1]+1]
}