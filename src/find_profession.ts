function solution(level: number, pos: number): string {
    let x = Math.pow(2,level-1) / 2
    let cnt = 0
    for (let i = 1; i < level; i++) {
        if (pos > x) { 
            pos -= x
            cnt++ 
        }
        x /= 2
    }
    return cnt%2===0 ? "Engineer" : "Doctor"
}