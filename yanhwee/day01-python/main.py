from sys import stdin
from itertools import accumulate, starmap, tee, islice
from functools import reduce
from operator import lt 

def take(n, iterable):
    return list(islice(iterable, n))

def pairwise(gap, iterable):
    a, b = tee(iterable)
    xs = take(gap, b)
    return xs, zip(a, b)

def iter_count(predicate, iterable):
    count = lambda c, x: c + 1 if predicate(x) else c
    return reduce(count, iterable, 0)

def count_inc(iterable):
    _, pairs = pairwise(1, iterable)
    return iter_count(bool, starmap(lt, pairs))

def moving_sum(window, iterable):
    start, pairs = pairwise(window, iterable)
    old = lambda pair: pair[0]
    new = lambda pair: pair[1]
    update = lambda acc, pair: acc - old(pair) + new(pair)
    return accumulate(pairs, update, initial=sum(start))

if __name__ == '__main__':
    lines = list(map(str.rstrip, stdin))
    nums = list(map(int, lines))
    print('Part 1', count_inc(nums))
    print('Part 2', count_inc(moving_sum(3, nums)))

