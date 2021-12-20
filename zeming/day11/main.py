from typing import List
from pprint import pprint

import sys
sys.setrecursionlimit(10000)

row, col = 10, 10
total_steps = 100
total_flashes = 0

def get_adj(x, y):
    pos = []
    sx = x if x-1<0 else x-1
    sy = y if y-1<0 else y-1
    ex = x if x+1>col-1 else x+1
    ey = y if y+1>row-1 else y+1
    for i in range(sx, ex+1):
        for j in range(sy, ey+1):
            if i == x and j == y:
                continue
            pos.append([i, j])
    return pos

def inc_flash(grid, grid_t, x, y):
    if grid[x][y] > 9:
        global total_flashes
        total_flashes += 1
        grid_t[x][y] = True
        for adj in get_adj(x, y):
            if grid_t[adj[0]][adj[1]] == False:
                grid[adj[0]][adj[1]] += 1
                inc_flash(grid, grid_t, adj[0], adj[1])
        grid[x][y] = 0

def inc(grid, grid_t):
    for i in range(row):
        for j in range(col):
            grid[i][j] += 1
            grid_t[i][j] = False
    points = check(grid)
    for p in points:
        inc_flash(grid, grid_t, p[0], p[1])

def check(grid) -> List[List[int]]:
    more = []
    for i in range(row):
        for j in range(col):
            if grid[i][j] > 9:
                more.append([i,j])
    return more

def p1():
    with open("in1", "r") as f:
        l = f.readlines()
        l = [line.rstrip() for line in l]
        ll = [[0 for i in range(row)] for j in range(col)]
        lll = [[False for i in range(row)] for j in range(col)]
        for i in range(row):
            for j in range(row):
                ll[i][j] = int(l[i][j])
        for _ in range(100):
            inc(ll, lll)
        pprint(ll)
        print("total flash: ", total_flashes)

p1()
