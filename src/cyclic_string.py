def solution(s):
    cnt = 1
    for _ in range(len(s)):
        a = s[0:cnt]*((len(s)//cnt)+1)
        if a[:len(s)] == s:
            break
        cnt+=1
    return cnt
