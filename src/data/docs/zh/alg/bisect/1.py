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
        res.append(bisect(a_ls, q-1))
    else:
        res.append(-1)
print(res)

# def bisect(a, x, lo=0, hi=None, key=lambda f: f):
#   if hi is None: hi = len(a)
#   while lo < hi:
#     mid = (lo + hi) >> 1
#     if x < key(a[mid]): hi = mid
#     else: lo = mid + 1
#   return lo