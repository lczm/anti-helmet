package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

type Op int64

const (
	Sum Op = iota
	Product
	Min
	Max
	Gt
	Lt
	Eq
	Literal
	Default
)

var versionSum int64
var stack []Op = make([]Op, 0)
var literalStack []int64 = make([]int64, 0)
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

	switch typeIDInt {
	case 4: // Literal evaluation
		return pLiteral(s[6:]) + 6
	case 0: // sum packets
		return pOperator(s[6:], Sum) + 6
	case 1: // product packets
		return pOperator(s[6:], Product) + 6
	case 2: // minimum packets
		return pOperator(s[6:], Min) + 6
	case 3: // maximum packets
		return pOperator(s[6:], Max) + 6
	case 5: // greater than packets
		return pOperator(s[6:], Gt) + 6
	case 6: // less than packets
		return pOperator(s[6:], Lt) + 6
	case 7: // equal to packets
		return pOperator(s[6:], Eq) + 6
	default:
		return pOperator(s[6:], Default) + 6
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
	// _ = literal

	literalStack = append(literalStack, literal)
	stack = append(stack, Literal)

	return ip
}

func pushOp(o Op) {
	stack = append(stack, o)
}

func pushLiteral(literal int64) {
	literalStack = append(literalStack, literal)
}

func pOperator(s string, o Op) int {
	ip := 0
	count := 0
	if s[ip] == '0' { // next 15-bits
		l := s[1:16]
		lInt, _ := strconv.ParseInt(l, 2, 64)
		count = 0
		for int64(count) != lInt {
			count += p(s[count+16:])
		}
		pushOp(o)
		return count + 16
	} else if s[ip] == '1' { // next 11-bits
		l := s[1:12]
		lInt, _ := strconv.ParseInt(l, 2, 64)
		count = 0
		for i := 0; int64(i) < lInt; i++ {
			count += p(s[count+12:])
		}
		pushOp(o)
		return count + 12
	} else {
		panic("error parsing operator")
	}
	return count
}

func popStack() Op {
	op := stack[len(stack)-1]
	stack = stack[:len(stack)-1]
	return op
}

func peekStack() Op {
	return stack[len(stack)-1]
}

func canPeekStack() bool {
	if len(stack) >= 1 {
		return true
	}
	return false
}

func popLiteralStack() int64 {
	literal := literalStack[len(literalStack)-1]
	literalStack = literalStack[:len(literalStack)-1]
	return literal
}

func min(a []int64) int64 {
	var m int64 = 9999999999
	for _, v := range a {
		if v < m {
			m = v
		}
	}
	return m
}

func max(a []int64) int64 {
	var m int64 = 0
	for _, v := range a {
		if v > m {
			m = v
		}
	}
	return m
}

func gt(a, b int64) int64 {
	if a > b {
		return 1
	}
	return 0
}

func lt(a, b int64) int64 {
	if a < b {
		return 1
	}
	return 0
}

func eq(a, b int64) int64 {
	if a == b {
		return 1
	}
	return 0
}

// Uses stack & literalStack
func eval(op Op) int64 {
	switch op {
	case Sum:
		var sum int64 = 0
		for canPeekStack() && peekStack() == Literal {
			sum += eval(popStack())
		}
		return sum
	case Product:
		var product int64 = 1
		for canPeekStack() && peekStack() == Literal {
			product *= eval(popStack())
		}
		return product
	case Min:
		val := make([]int64, 0)
		for canPeekStack() && peekStack() == Literal {
			val = append(val, eval(popStack()))
		}
		return min(val)
	case Max:
		val := make([]int64, 0)
		for canPeekStack() && peekStack() == Literal {
			val = append(val, eval(popStack()))
		}
		return max(val)
	case Gt:
		lit2, lit1 := eval(popStack()), eval(popStack())
		return gt(lit1, lit2)
	case Lt:
		lit2, lit1 := eval(popStack()), eval(popStack())
		return lt(lit1, lit2)
	case Eq:
		lit2, lit1 := eval(popStack()), eval(popStack())
		return eq(lit1, lit2)
	case Literal:
		lit := popLiteralStack()
		return lit
	}
	return 0
}

func main() {
	// part1
	// inFiles := []string{"in1", "in2", "in3", "in4", "in5", "in6", "in7", "in8"}
	// part2
	inFiles := []string{"p1", "p2", "p3", "p4", "p5", "p6", "p7", "p8", "in1"}
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
		r := eval(popStack())

		fmt.Println("#", in, "versionSum:", versionSum, s, len(bin), r)

		// reset all
		versionSum = 0
		stack = make([]Op, 0)
		literalStack = make([]int64, 0)
	}
}
