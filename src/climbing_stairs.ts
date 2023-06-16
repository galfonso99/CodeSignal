function solution(n: number): number {
    let arr = [1,2,3]
    for (let i = 3; i<n; i++) {
        arr.push(arr[i-1] + arr[i-2])
    }
    return arr[n-1]
}

//Fibonacci