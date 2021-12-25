package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

type Cave struct {
	Name  string
	Large bool
	Edges []string
}

func findPaths(currentCave string,
	caves map[string]*Cave, path []string, revisited bool) [][]string {
	if currentCave == "end" {
		path = append(path, currentCave)
		return [][]string{path}
	}

	if !caves[currentCave].Large {
		for _, c := range path {
			if currentCave == c {
				if currentCave == "start" {
					return [][]string{}
				}
				if revisited {
					return [][]string{}
				}
				revisited = true
			}
		}
	}

	paths := [][]string{}
	path = append(path, currentCave)
	for _, c := range caves[currentCave].Edges {
		// Copy slice to prevent corruption
		newPath := append([]string{}, path...)
		paths = append(paths, findPaths(c, caves, newPath, revisited)...)
	}
	return paths
}

func main() {
	body, err := ioutil.ReadFile("in1")
	if err != nil {
		panic("error reading file")
	}
	routes := strings.Split(
		strings.TrimSpace(string(body)), "\n")

	caves := map[string]*Cave{}

	for _, line := range routes {
		split := strings.Split(line, "-")
		left, right := split[0], split[1]

		cave, ok := caves[left]
		if !ok { // Not present
			large := false
			if left == strings.ToUpper(left) {
				large = true
			}
			cave = &Cave{left, large, []string{}}
			caves[cave.Name] = cave
		}
		cave.Edges = append(cave.Edges, right)

		cave, ok = caves[right]
		if !ok {
			isLarge := false
			if right == strings.ToUpper(right) {
				isLarge = true
			}
			cave = &Cave{right, isLarge, []string{}}
			caves[cave.Name] = cave
		}
		cave.Edges = append(cave.Edges, left)
	}

	paths := findPaths("start", caves, []string{}, false)
	fmt.Println(len(paths))
}
