def solution(num, h):
    st = ["*", "*", "***"]
    for l in range(1, num+1):
        for i in range(1, h+1):
            n= 2*i+2*l+1
            st.append("*"*n)
    for _ in range(1,num+1):
        n = h + (1 if h%2==0 else 0)
        st.append("*"*n)
    n = 2*h+2*num+1
    st = [s.center(n,' ').rstrip() for s in st]
    return st