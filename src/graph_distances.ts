function solution(g: number[][], s: number): number[] {
    let visited = Array(g.length).fill(false)
    let dist = Array(g.length).fill(-1)
    dist[s] = 0
    while (!visited[s]) {
        visited[s] = true
        let min = 9e7
        let ind = 0
        for (let i = 0; i < g.length; i++) {
            if (g[s][i] < 0 || visited[i]) {continue}
            if (dist[i] > g[s][i] + dist[s] || dist[i] < 0) {
                dist[i] = g[s][i] + dist[s]
            }
            if (dist[i] < min) {min = dist[i]; ind = i}
        }
        s = ind
        
    }
    return dist
}