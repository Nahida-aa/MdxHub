import sys
input = lambda: sys.stdin.readline().strip()
n, m = map(int, input().split())
matrix = [list(map(int, input().split())) for _ in range(n)]
print(matrix)
