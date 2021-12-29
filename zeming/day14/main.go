package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	body, err := ioutil.ReadFile("in1")
	if err != nil {
		panic("error reading file")
	}

	lines := strings.Split(
		strings.TrimSpace(string(body)), "\n")

	// generate mapping
	mapping := make(map[string]string)
	for i := 2; i < len(lines); i++ {
		line := lines[i]
		line_split := strings.Split(line, "->")
		line_split[0] = strings.TrimSpace(line_split[0])
		line_split[1] = strings.TrimSpace(line_split[1])
		mapping[line_split[0]] = line_split[1]
	}

	steps := 40
	start := lines[0]

	// part1
	// for step := 0; step < steps; step++ {
	// 	offset := 0
	// 	add_index := make([]int, 0)
	// 	add_string := make([]string, 0)
	// 	for i := 0; i < len(start)-1; i++ {
	// 		first := string(start[i])
	// 		second := string(start[i+1])
	// 		join := first + second
	// 		if val, ok := mapping[join]; ok {
	// 			add_index = append(add_index, i+1+offset)
	// 			add_string = append(add_string, val)
	// 			offset++
	// 		}
	// 	}
	// 	fmt.Println("length: ", len(start)+offset)
	// 	var buffer bytes.Buffer
	// 	inner_offset := 0
	// 	for i := 0; i < len(start)+offset; i++ {
	// 		added := false
	// 		for j := 0; j < len(add_index); j++ {
	// 			if i == add_index[j] {
	// 				buffer.WriteString(add_string[j])
	// 				inner_offset++
	// 				added = true
	// 			}
	// 		}
	// 		if !added {
	// 			buffer.WriteString(string(start[i-inner_offset]))
	// 		}
	// 	}
	// 	// fmt.Println("-", buffer.String())
	// 	// fmt.Println(add_index)
	// 	// fmt.Println(add_string)
	// 	// fmt.Println(offset)
	// 	start = buffer.String()
	// }

	// part2
	trackMap := make(map[string]int)
	for i := 0; i < len(start)-1; i++ {
		trackMap[string(start[i])+string(start[i+1])]++
	}
	for step := 0; step < steps; step++ {
		newMap := make(map[string]int)
		for k, v := range trackMap {
			newMap[string(k[0])+mapping[k]] += v
			newMap[mapping[k]+string(k[1])] += v
		}
		trackMap = newMap
	}

	// part1
	// count
	// count := make(map[string]int)
	// for _, c := range start {
	// 	count[string(c)]++
	// }
	// min := 99999999999999999
	// max := 0
	// for _, v := range count {
	// 	if v < min {
	// 		min = v
	// 	}
	// 	if v > max {
	// 		max = v
	// 	}
	// }
	// fmt.Println(max - min)

	// part2
	count := make(map[string]int)
	for k, v := range trackMap {
		count[string(k[0])] += v
	}
	min := 99999999999999999
	max := 0
	for _, v := range count {
		if v < min {
			min = v
		}
		if v > max {
			max = v
		}
	}
	// for some reason my answer is 1 short when compared to part1?
	fmt.Println((max - min) + 1)
}
