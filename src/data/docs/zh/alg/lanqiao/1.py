words = ['lanqiao', 'wangyi', 'tencent', 'baidu', 'alibaba']
print(sorted(words, key=lambda x: x[0]))  # ['alibaba', 'baidu', 'lanqiao', 'tencent', 'wangyi']
print(sorted(words, key=len)) # ['baidu', 'lanqiao', 'wangyi', 'tencent', 'alibaba']

n = 20
[x for x in range(2, n + 1) 
    if all(
        x % i != 0 for i in range(2, int(x**0.5) + 1)
    )
]
