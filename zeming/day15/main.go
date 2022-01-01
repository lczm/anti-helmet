package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"strconv"
	"strings"
)

type Coordinate struct {
	X, Y int
}

func printGrid(path []Coordinate, min, max Coordinate) {
	maxX, maxY := max.X, max.Y
	grid := make([][]string, maxX+1)
	for i := range grid {
		grid[i] = make([]string, maxY+1)
	}
	for _, p := range path {
		grid[p.Y][p.X] = "*"
	}
	for i := 0; i <= maxY; i++ {
		for j := 0; j <= maxX; j++ {
			if grid[i][j] != "*" {
				grid[i][j] = "."
			}
		}
	}
	for _, g := range grid {
		fmt.Println(g)
	}
}

func getNeighbour(c, min, max Coordinate) []Coordinate {
	coordinates := make([]Coordinate, 0)
	x, y := c.X, c.Y
	minX, minY := min.X, min.Y
	maxX, maxY := max.X, max.Y
	if x != minX {
		coordinates = append(coordinates, Coordinate{x - 1, y})
	}
	if x != maxX {
		coordinates = append(coordinates, Coordinate{x + 1, y})
	}
	if y != minY {
		coordinates = append(coordinates, Coordinate{x, y - 1})
	}
	if y != maxY {
		coordinates = append(coordinates, Coordinate{x, y + 1})
	}
	return coordinates
}

func dijkstra(grid [][]int,
	start, min, max Coordinate) []Coordinate {
	// initialize all
	queue := make([]Coordinate, 0)
	visited := make(map[Coordinate]bool)
	inqueue := make(map[Coordinate]bool)
	distance := make(map[Coordinate]int)
	path := make(map[Coordinate]Coordinate)
	for i := min.X; i <= max.X; i++ {
		for j := min.Y; j <= max.Y; j++ {
			visited[Coordinate{i, j}] = false
			distance[Coordinate{i, j}] = math.MaxInt
			path[Coordinate{i, j}] = Coordinate{-1, -1}
		}
	}

	// enqueue start to the queue
	queue = append(queue, start)
	distance[start] = grid[start.Y][start.X]
	for len(queue) != 0 {
		// Get element and set visited
		e := queue[0]
		visited[e] = true
		queue = queue[1:] // pop the first element

		neighbours := getNeighbour(e, min, max)
		for _, neighbour := range neighbours {
			// if not visited and not already in queue
			if !visited[neighbour] {
				if !inqueue[neighbour] {
					queue = append(queue, neighbour)
					inqueue[neighbour] = true
				}
				if distance[neighbour] > distance[e]+grid[neighbour.Y][neighbour.X] {
					path[neighbour] = e
					distance[neighbour] = distance[e] + grid[neighbour.Y][neighbour.X]
				}
			}
		}
	}

	end := Coordinate{max.Y, max.X}
	shortestPath := make([]Coordinate, 0)
	for {
		shortestPath = append(shortestPath, end)
		end = path[end]
		if end == start {
			shortestPath = append(shortestPath, end)
			break
		}
	}

	// reverse the shortest path
	for i, j := 0, len(shortestPath)-1; i < j; i, j = i+1, j-1 {
		shortestPath[i], shortestPath[j] = shortestPath[j], shortestPath[i]
	}

	return shortestPath
}

func main() {
	body, err := ioutil.ReadFile("in1")
	if err != nil {
		panic("error reading file")
	}

	lines := strings.Split(
		strings.TrimSpace(string(body)), "\n")
	length, height := len(lines[0]), len(lines)
	grid := make([][]int, length)
	for i := range grid {
		grid[i] = make([]int, height)
	}
	for i, line := range lines {
		for j, c := range line {
			val, err := strconv.Atoi(string(c))
			if err != nil {
				panic("error converting char rune to int")
			}
			grid[i][j] = val
		}
	}

	start := Coordinate{0, 0}
	min := Coordinate{0, 0}
	max := Coordinate{len(grid) - 1, len(grid[0]) - 1}

	path := dijkstra(grid, start, min, max)
	risk := 0
	for _, c := range path {
		// fmt.Println(c)
		if c == start {
			// fmt.Println("skipping")
			continue
		}
		risk += grid[c.Y][c.X]
	}
	fmt.Println("part1:", risk)
	// printGrid(path, min, max)

	// part 2
	grid2 := make([][]int, length*5)
	for i := range grid2 {
		grid2[i] = make([]int, height*5)
	}
	for x := 0; x < 5; x++ {
		for y := 0; y < 5; y++ {
			for i, line := range lines {
				for j, c := range line {
					val, err := strconv.Atoi(string(c))
					val = (val + x + y) % 9
					if val == 0 {
						val = 9
					}
					if err != nil {
						panic("error converting char rune to int")
					}
					grid2[i+(x*height)][j+(y*length)] = val
				}
			}
		}
	}
	for _, g := range grid2 {
		fmt.Println(g)
	}
	max2 := Coordinate{len(grid2) - 1, len(grid2[0]) - 1}
	path2 := dijkstra(grid2, start, min, max2)
	risk2 := 0
	for _, c := range path2 {
		if c == start {
			continue
		}
		risk2 += grid2[c.Y][c.X]
		// fmt.Println(risk2)
	}
	fmt.Println("part2:", risk2)
	// printGrid(path2, min, max2)
}
