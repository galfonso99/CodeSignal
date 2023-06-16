function solution(file: string): number {
    let lvl = 0
    let dirs = file.split('\f')
    let lengths :number[][] = []
    dirs.reverse()
    for (let s of dirs) {
        let split = s.split('\t')
        let dp = split.length - 1
        if (split[dp].includes('.')) {
            lengths.push([dp, split[dp].length])
            lvl = dp
        }
        else if (dp < lvl) {
            for (let i = 0 ; i<lengths.length; i++) {
                if (lengths[i][0] > dp) {
                    lengths[i][1] += split[dp].length+1
                    lengths[i][0]--
                }
            }
            lvl = dp
        }
        else if (dp > lvl) { lvl = dp }
    }
    if (lengths.length < 1) {return 0}
    let maxx = lengths.map((arr) => arr[1]).reduce((prev,cur) => cur > prev ? cur : prev)
    return maxx
}