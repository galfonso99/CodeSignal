function solution(s,l) {
    r=new Date(new Date(s)-(new Date(l)-new Date(s))).toISOString()
    return r.slice(0,10)+" "+r.slice(11,16)
}