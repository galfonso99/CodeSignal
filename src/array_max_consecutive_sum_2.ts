function solution(a: number[]): number {
    let global_total = a[0]
    let local_total = 0
    for (let i = 0; i < a.length; i++) {
        if (a[i] > local_total + a[i]) {
            local_total = a[i]
        }
        else {
            local_total += a[i]
        }
        if (local_total > global_total) {
            global_total = local_total
        }
    }
    return global_total
}