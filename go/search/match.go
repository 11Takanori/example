package main

import "fmt"

func bf_match(txt, pat string) int {
	var pt, pp int

	for pt <= len(txt)-1 && pp <= len(pat)-1 {
		if txt[pt] == pat[pp] {
			pt++
			pp++
		} else {
			pt = pt - pp + 1
			pp = 0
		}
	}
	if pp == len(pat) {
		return pt - pp
	}
	return -1
}

func main() {
	var s1, s2 string
	var idx int

	fmt.Println("text: ")
	fmt.Scan(&s1)

	fmt.Println("patternxt: ")
	fmt.Scan(&s2)

	idx = bf_match(s1, s2)

	if idx == -1 {
		fmt.Println("not match")
	} else {
		fmt.Printf("%d\n", idx+1)
	}
}
