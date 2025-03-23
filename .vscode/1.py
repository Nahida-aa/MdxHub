import sys
input = lambda: sys.stdin.readline().strip()
matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
flatten = [item for row in matrix for item in row]
print(flatten)

