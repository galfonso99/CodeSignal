def solution(s):
    st = []
    temp = [s[0]]
    for i in range(len(s)-1):
        if s[i] == s[i+1]:
            temp.append(s[i+1])
        else:
            st.append(''.join(temp))
            temp = [s[i+1]]
    st.append(''.join(temp))
    for i,k in enumerate(st):
        if k.count(k[0]) > 1:
            st[i] = str(k.count(k[0]))+k[0]
        else:
            st[i] = k[0]      
    return ''.join(st)
