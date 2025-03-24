import sys
input = lambda: sys.stdin.readline().strip()
n, S = map(int, input().split())
matrix_p_c = [[0,0]] * n
for i in range(n):
    matrix_p_c[i] = list(map(int, input().split()))
# 排序
p, c = [0] * n, [0] * n
matrix_p_c.sort(key=lambda x: x[1])
for i in range(n):
    p[i], c[i] = matrix_p_c[i][0], matrix_p_c[i][1]
# 初始化
res, cnt, tot = 0, 0, sum(p)
# 遍历
for i in range(n):
    if S <= tot:
        res += (c[i] - cnt) * S
        cnt = c[i]
    else:
        res += (c[i] - cnt) * p[i]
    tot -= p[i]
print(res)
