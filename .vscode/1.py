import sys
input = lambda: sys.stdin.readline().strip()
def sieve_of_eratosthenes(n):
    is_prime = [True] * (n + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, int(n**0.5) + 1): # i*i <= n
        if is_prime[i]: # e.g if 3 是素数, 那么 3*3, 3*4, 3*5... 都不是素数
            for j in range(i * i, n + 1, i):
                is_prime[j] = False
    return [x for x in range(3,n + 1) if is_prime[x]]
primes = sieve_of_eratosthenes(20)
print(primes)  # [2, 3, 5, 7, 11, 13, 17, 19]

