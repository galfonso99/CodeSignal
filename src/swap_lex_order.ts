function solution(str: string, pairs: number[][]): string {
    if (pairs.length < 1) return str;
    let pairMap = {};
    pairs.forEach(pair => {
        pairMap[pair[0]] = pairMap[pair[0]] ? pairMap[pair[0]].concat([pair[1]]) : [pair[1]];
        pairMap[pair[1]] = pairMap[pair[1]] ? pairMap[pair[1]].concat([pair[0]]) : [pair[0]];
    });
    
    let i, num, groups = [], group;
    for (num in pairMap) {
        group = [+num].concat(pairMap[num]);
        for (i = 1; i < group.length; i++) {
            pairMap[group[i]].forEach(n => {
                if (!group.includes(+n)) { group.push(+n) }
            });
            delete pairMap[group[i]];
        }
        groups.push(group);
    }
    
    let outArray = [];
    let grpChars;
    groups.forEach(group => {
        group.sort((a, b) => a - b);
        grpChars = [];
        group.forEach(index => grpChars.push(str.charAt(index-1)));
        grpChars.sort().reverse();
        group.forEach((strIndex, grpIndex) => {
            outArray[strIndex-1] = grpChars[grpIndex];
        });
    });
    for (i = 0; i < str.length; i++) {
        if (outArray[i] == null) {
            outArray[i] = str.charAt(i);
        }
    }
    
    return outArray.join('');
}