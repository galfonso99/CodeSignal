function solution(dishes: string[][]): string[][] {
    let map = new Map()
    let arr = new Array()
    for (let arr of dishes) {
        for (let i = 1; i<arr.length;i++) {
            if (!map.get(arr[i])) {
                map.set(arr[i], [arr[0]])
            }
            else {
                map.set(arr[i], [...map.get(arr[i]), arr[0]])
            }
        }
    }
    for (let [key,array] of map) {
        if (array.length > 1) {
            array.sort()
            //array.sort((a, b) => a < b ? -1 : a > b ? 1 :0)
            arr.push([key, ...array])
        }
    }
    arr.sort((a, b) => a[0] < b[0] ? -1 : a[0] > b[0] ? 1 :0)
    return arr
}