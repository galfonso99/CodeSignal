def solution(p, s):
    l = len(s)
    a = []
    for i in range(l-1):
        a.extend([(p[i]-p[j])/(s[j]-s[i]) for j in range(i+1, l) if s[j]-s[i]!=0])
    b = [a.count(i) for i in set(a) if i >= 0]
    return math.sqrt(max(b)*2+1/4)+1/2 if b != [] else -1