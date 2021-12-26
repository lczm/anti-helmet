package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func p(grid [][]bool) {
	s := make([][]string, len(grid))
	for i := range s {
		s[i] = make([]string, len(grid[0]))
	}

	for i := 0; i < len(grid); i++ {
		for j := 0; j < len(grid[0]); j++ {
			if grid[i][j] {
				s[i][j] = "*"
			} else {
				s[i][j] = "."
			}
		}
	}

	for _, s2 := range s {
		fmt.Println(s2)
	}
}

func main() {
	body, err := ioutil.ReadFile("in1")
	if err != nil {
		panic("error reading file")
	}
	lines := strings.Split(
		strings.TrimSpace(string(body)), "\n")

	max_x := 0
	max_y := 0
	fold_index := 0
	xs := make([]int, 0)
	ys := make([]int, 0)
	for i, line := range lines {
		if line == "" {
			fold_index = i
			break
		}
		split := strings.Split(line, ",")
		x, _ := strconv.Atoi(split[0])
		y, _ := strconv.Atoi(split[1])
		// fmt.Println(x, y)
		if x > max_x {
			max_x = x
		}
		if y > max_y {
			max_y = y
		}
		xs = append(xs, x)
		ys = append(ys, y)
	}
	fmt.Println("max: ", max_x, max_y)
	max_x++ // 0-index count
	max_y++ // 0-index count

	// A little inefficient, but build the 2d array
	grid := make([][]bool, max_y)
	for i := range grid {
		grid[i] = make([]bool, max_x)
	}
	if len(xs) != len(ys) {
		panic("not the same amount of coordinates, something went wrong")
	}
	for i := 0; i < len(xs); i++ {
		grid[ys[i]][xs[i]] = true
	}

	// start folding
	// fold_index + 1 to skip the blank line
	for _, line := range lines[fold_index+1:] {
		if !strings.Contains(line, "fold along") {
			panic("not at fold something went wrong")
		}
		eq_index := strings.Index(line, "=")
		xy := string(line[eq_index-1])
		val, _ := strconv.Atoi(string(line[eq_index+1:]))
		var fold_horizontally bool
		if xy == "x" {
			fold_horizontally = false
		} else if xy == "y" {
			fold_horizontally = true
		}

		if fold_horizontally {
			fmt.Println("folding horizontally")
			for y := val + 1; y < max_y; y++ {
				for x := 0; x < max_x; x++ {
					n := grid[y][x]
					if n {
						grid[max_y-y-1][x] = true
					}
				}
			}
			max_y = val
			grid = grid[:val]
		} else { // fold vertically
			// part1
			fmt.Println("folding vertically")
			for x := val + 1; x < max_x; x++ {
				for y := 0; y < max_y; y++ {
					n := grid[y][x]
					if n {
						grid[y][max_x-x-1] = true
					}
				}
			}
			max_x = val
			for i := 0; i < len(grid); i++ {
				grid[i] = grid[i][:val]
			}
		}
	}

	p(grid)
}
