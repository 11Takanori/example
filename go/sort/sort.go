package main

import "fmt"

func bubble1(a []int) {
	for i := 0; i < len(a)-1; i++ {
		for j := len(a) - 1; j > i; j-- {
			if a[j-1] > a[j] {
				a[j-1], a[j] = a[j], a[j-1]
			}
		}
	}
}

func bubble2(a []int) {
	for i := 0; i < len(a)-1; i++ {
		exchg := 0
		for j := len(a) - 1; j > i; j-- {
			if a[j-1] > a[j] {
				a[j-1], a[j] = a[j], a[j-1]
				exchg++
			}
			if exchg == 0 {
				break
			}
		}
	}
}

func bubble3(a []int) {
	last := len(a) - 1
	for i := 0; i < len(a)-1; {
		for j := len(a) - 1; j > i; j-- {
			if a[j-1] > a[j] {
				a[j-1], a[j] = a[j], a[j-1]
				last = j
			}
		}
		i = last
	}
}

func insertion(a []int) {
	for i := 1; i < len(a); i++ {
		tmp := a[i]
		var j int
		for j = i; j > 0 && a[j-1] > tmp; j-- {
			a[j] = a[j-1]
		}
		a[j] = tmp
	}
}

func selection(a []int) {
	for i := 0; i < len(a)-1; i++ {
		min := i
		for j := i + 1; j < len(a); j++ {
			if a[j] < a[min] {
				min = j
			}
		}
		a[i], a[min] = a[min], a[i]
	}
}

func main() {
	var n int

	fmt.Println("item number: ")
	fmt.Scan(&n)
	a := make([]int, n)

	for i := 0; i < n; i++ {
		fmt.Printf("a[%d] : ", i)
		fmt.Scanf("%d", &a[i])
	}

	selection(a)
	fmt.Printf("%v\n", a)
}
