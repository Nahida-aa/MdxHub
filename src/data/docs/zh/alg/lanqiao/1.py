words = ['lanqiao', 'wangyi', 'tencent', 'baidu', 'alibaba']
print(sorted(words, key=lambda x: x[0]))  # ['alibaba', 'baidu', 'lanqiao', 'tencent', 'wangyi']
print(sorted(words, key=len)) # ['baidu', 'lanqiao', 'wangyi', 'tencent', 'alibaba']