import sys
input = lambda: sys.stdin.readline().strip()
n, m = map(int, input().split())
a_ls = list(map(int, input().split()))
q_ls = list(map(int, input().split()))
from bisect import bisect
from typing import List

s = set(a_ls)
res = []
for q in q_ls:
    if q in s:
        res.append(bisect(a_ls, q-1)+1)
    else:
        res.append(-1)
print(*res)


