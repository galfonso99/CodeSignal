function solution(message: string): number {
    if (message.length===0) {return 1}
    let fst = message[message.length-1] === '0' ? 0 : 1,
    scd = encodings(message[message.length-2], message[message.length-1])
    let partial = [fst, scd]
    for (let i = message.length-3; i>-1; i--) {
        let enc = encodings(message[i], message[i+1])
        if (enc === -1) {return 0}
        else if (enc === 0) {partial.push(0)}
        else if (parseInt((message[i]+message[i+1]))>26) {partial.push(partial[message.length-2-i])}
        else {partial.push((partial[message.length-2-i]+partial[message.length-3-i])%(1e9+7))}
    }
    return partial[message.length-1]
}

const encodings = (a,b: string) :number => {
    if (b ==='0' && (a === '0' || a > '2')) {return -1}
    else if (a==='0') {return 0}
    else if ( parseInt(a+b) > 26 || b === '0') {return 1}
    return 2
}