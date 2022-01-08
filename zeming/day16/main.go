package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

var versionSum int64
var table map[string]string = map[string]string{

	"0": "0000",
	"1": "0001",
	"2": "0010",
	"3": "0011",
	"4": "0100",
	"5": "0101",
	"6": "0110",
	"7": "0111",
	"8": "1000",
	"9": "1001",
	"A": "1010",
	"B": "1011",
	"C": "1100",
	"D": "1101",
	"E": "1110",
	"F": "1111",
}

func hexToBin(s string, table map[string]string) string {
	n := ""
	for _, c := range s {
		n += table[string(c)]
	}
	return n
}

func p(s string) int {
	return pHeader(s)
}

func pHeader(s string) int {
	// Eat the version and type
	version := s[:3]
	typeID := s[3:6]

	versionInt, _ := strconv.ParseInt(version, 2, 64)
	typeIDInt, _ := strconv.ParseInt(typeID, 2, 64)

	// Increment the version sum used for part 1
	versionSum += versionInt

	if typeIDInt == 4 { // Literal
		return pLiteral(s[6:]) + 6
	} else {
		return pOperator(s[6:]) + 6
	}
}

func pLiteral(s string) int {
	ip := 0
	builder := ""

	for {
		if s[ip] == '1' { // not last
			builder += s[ip : ip+5]
			ip += 5
		} else {
			builder += s[ip : ip+5]
			ip += 5
			break
		}
	}

	literal, _ := strconv.ParseInt(builder, 2, 64)
	_ = literal
	_ = builder

	return ip
}

func pOperator(s string) int {
	ip := 0
	count := 0
	if string(s[ip]) == "0" { // next 15-bits
		l := s[1:16]
		lInt, _ := strconv.ParseInt(l, 2, 64)
		count = 0
		for int64(count) != lInt {
			count += p(s[count+16:])
		}
		return count + 16
	} else if string(s[ip]) == "1" { // next 11-bits
		l := s[1:12]
		lInt, _ := strconv.ParseInt(l, 2, 64)
		count = 0
		for i := 0; int64(i) < lInt; i++ {
			count += p(s[count+12:])
		}
		return count + 12
	} else {
		panic("error parsing operator")
	}
	return count
}

func main() {
	inFiles := []string{"in1", "in2", "in3", "in4", "in5", "in6", "in7", "in8"}
	for _, in := range inFiles {
		body, _ := ioutil.ReadFile(in)
		lines := strings.Split(
			strings.TrimSpace(string(body)), "\n")
		if len(lines) > 1 {
			panic("input has more than one line")
		}

		line := lines[0]
		bin := hexToBin(line, table)
		s := p(bin)

		fmt.Println("#", in, "versionSum:", versionSum, s, len(bin))
		versionSum = 0
	}
}
