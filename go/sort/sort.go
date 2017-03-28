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

func shell(a []int) {
	var i, j, h int
	for h = len(a) / 2; h > 0; h /= 2 {
		for i = h; i < len(a); i++ {
			tmp := a[i]
			for j = i - h; j >= 0 && a[j] > tmp; j -= h {
				a[j+h] = a[j]
			}
			a[j+h] = tmp
		}
	}

}

func quick(a []int, left, right int) {
	pl := left
	pr := right
	x := a[(pl+pr)/2]

	for {
		for a[pl] < x {
			pl++
		}
		for a[pr] > x {
			pr--
		}
		if pl <= pr {
			a[pl], a[pr] = a[pr], a[pl]
			pl++
			pr--
		}
		if pl > pr {
			break
		}
	}
	if left < pr {
		quick(a, left, pr)
	}
	if pl < right {
		quick(a, pl, right)
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

	shell(a)
	fmt.Printf("%v\n", a)
}
