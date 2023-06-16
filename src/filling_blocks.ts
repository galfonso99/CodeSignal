function solution(n: number): number {
    let partial = [1,1,5,11]  //Derived empirically  
    for (let i = 4; i < n+1; i++) {
        partial.push( partial[i-1] + partial[i-2] * 5 + partial[i-3] - partial[i-4] )
    }
    return partial[n]
}