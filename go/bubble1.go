package main

import "fmt"

func bubble(a []int) {
	for i := 0; i < len(a)-1; i++ {
		for j := len(a) - 1; j > i; j-- {
			if a[j-1] > a[j] {
				a[j-1], a[j] = a[j], a[j-1]
			}
		}
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

	bubble(a)
	fmt.Printf("%v\n", a)
}
