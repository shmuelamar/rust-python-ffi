from itertools import takewhile


def fib():
    a, b = 0, 1
    while True:
        yield a
        a, b = b, a + b


def sum_fib_even_functional(max_num):
    seq = takewhile(lambda x: x < max_num, fib())
    return sum(filter(lambda x: x % 2 == 0, seq))


def sum_fib_even(max_num):
    a, b, s = 0, 1, 0
    while a < max_num:
        if a % 2 == 0:
            s += a
        a, b = b, a+b
    return s


def sum_fib_10k(max_num):
    for i in range(10000):
        res = sum_fib_even(max_num)
    return res


def sum_fib_10k_fun(max_num):
    for i in range(10000):
        res = sum_fib_even_functional(max_num)
    return res
