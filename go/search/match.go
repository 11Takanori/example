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

func kmp_match(txt, pat string) int {
	pt, pp := 1, 0
	skip := make([]int, 1024)

	for pt < len(pat)-1 {
		if pat[pt] == pat[pp] {
			skip[pt] = pp
			pt++
			pp++
		} else if pp == 0 {
			skip[pt] = pp
			pt++
		} else {
			pp = skip[pp]
		}
	}

	pt, pp = 0, 0
	for pt <= len(txt)-1 && pp <= len(pat)-1 {
		if txt[pt] == pat[pp] {
			pt++
			pp++
		} else if pp == 0 {
			pt++
		} else {
			pp = skip[pp]
		}
	}

	if pp > len(pat)-1 {
		return pt - pp
	}

	return -1
}

func bm_match(txt, pat string) int {
	pt, pp := 0, 0
	txt_len := len(txt)
	pat_len := len(pat)
	skip := make([]int, 1024)

	for pt := 0; pt < 1024; pt++ {
		skip[pt] = pat_len
	}
	for pt := 0; pt < pat_len; pt++ {
		skip[pat[pt]] = pat_len - pt - 1
	}

	for pt < txt_len {
		pp = pat_len - 1
		for pt < txt_len {
			if pp == 0 {
				return pt
			}
			pp--
			pt--
		}
		if skip[txt[pt]] > pat_len-pp {
			pt += skip[txt[pt]]
		} else {
			pt += pat_len - pp
		}
	}
	return -1
}

func main() {
	var s1, s2 string
	var idx int

	fmt.Println("text: ")
	fmt.Scan(&s1)

	fmt.Println("pattern: ")
	fmt.Scan(&s2)

	idx = bm_match(s1, s2)

	if idx == -1 {
		fmt.Println("not match")
	} else {
		fmt.Printf("%d\n", idx+1)
	}
}
