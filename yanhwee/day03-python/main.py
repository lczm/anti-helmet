from sys import stdin
from itertools import count
from functools import reduce, partial
from operator import pow, add

def b2d(binary_num):
    p2 = map(partial(pow, 2), count())
    bs = map(bool, reversed(binary_num))
    return reduce(
        lambda acc, bp: acc + bp[1] if bp[0] else acc,
        zip(bs, p2), 0
    )

def part1(binary_nums):
    n = len(binary_nums)
    ns = reduce(partial(map, add), binary_nums)
    gamma = list(map(lambda x: int(x > n // 2), ns))
    epsilon = list(map(lambda b: 1 - b, gamma))
    gamma = b2d(gamma)
    epsilon = b2d(epsilon)
    return gamma * epsilon

if __name__ == '__main__':
   lines = list(map(str.strip, stdin))
   binary_nums = list(map(partial(map, int), lines))
   print('Part 1', part1(binary_nums))
