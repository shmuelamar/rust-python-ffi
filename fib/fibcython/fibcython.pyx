def sum_fib_10k(unsigned long n):
    cdef unsigned long i, res

    for i in range(10000):
        res = sum_fib_even(n)

    return res


cdef unsigned long sum_fib_even(unsigned long n):
    """sum even fibonacci numbers under `n`"""
    cdef unsigned long a = 0, b = 1, s = 0

    while a < n:
        a, b = b, a + b
        if a % 2 == 0:
            s += a
    return s
