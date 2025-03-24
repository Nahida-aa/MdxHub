# import sys
# input = lambda: sys.stdin.readline().strip()
# from typing import List

# people: List[List[int]] = [[7,0],[4,4],[7,1],[5,0],[6,1],[5,2]]
# # 1. 先对 h_i 降序排序, 对于 h_i 相同的情况, 对 k_i 升序排序
# people.sort(key=lambda x: (-x[0], x[1]))
# # 2. 
# res = []
# for i,v in enumerate(people):
#    h, k = v[0], v[1]
#    if k >= len(res):
#        res.append(v) # 这样虽然不能保证满足要求, 但是题目说了是数据是来自正确的队列, 打乱后的, 因为等其他插入完成后, 最终就能还原了
#    else:
#        res.insert(k, v) # 插入到 k 的位置, 这样前面就有 k 个大于等于 h 的元素
# print(res)

from bisect import bisect  # bisect is used for binary search operations on sorted lists
import builtins # 本身就会导入的模块: from builtins import *
a = [1,9,9,9,200,500]
print(bisect(a, 3)) # 1，a[1] = 9
print(bisect(a, 9)) # 4，a[4] = 200
print(bisect(a, -1)) # 0，a[0] = 1
print(bisect(a, 1000)) # 6，a[6]: error

a.reverse() # [500, 200, 9, 9, 9, 1]
# 找到第一个小于 x 的索引
a_rev_ = [-i for i in a] # [-500, -200, -9, -9, -9, -1]
print(bisect(a_rev_, -3)) # 5，a_[5] = -1
print(bisect(a_rev_, -9)) # 5，a_[5] = -1
print(bisect(a_rev_, -1)) # 6，a_[6]: err
print(bisect(a_rev_, -1000)) # 0，a_[0] = -500