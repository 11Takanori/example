package main

import "fmt"

func bubble(a []int) {
	for i := range a {
		for j := 0; j > i; j++ {
			if a[j] > a[j+1] {
				a[j], a[j+1] = a[j+1], a[j]
			}
		}
	}
	fmt.Println(a)
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
}
