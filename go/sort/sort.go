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

func shell2(a []int) {
	var i, j, h int
	for h = 1; h < len(a)/9; h = h*3 + 1 {
	}
	for ; h > 0; h /= 3 {
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

func merge(a []int, b []int) []int {
	r := make([]int, len(a)+len(b))
	i := 0
	j := 0

	for i < len(a) && j < len(b) {
		if a[i] <= b[j] {
			r[i+j] = a[i]
			i++
		} else {
			r[i+j] = b[j]
			j++
		}
	}

	for i < len(a) {
		r[i+j] = a[i]
		i++
	}
	for j < len(b) {
		r[i+j] = b[j]
		j++
	}

	return r
}

func Mergesort(a []int) []int {
	if len(a) < 2 {
		return a
	}

	middle := len(a) / 2
	former := Mergesort(a[:middle])
	latter := Mergesort(a[middle:])

	return merge(former, latter)
}

func downheap(a []int, left, right int) {
	var parent, child int
	temp := a[left]

	for parent = left; parent < (right+1)/2; parent = child {
		cl := parent*2 + 1
		cr := cl + 1

		if cr <= right && a[cr] > a[cl] {
			child = cr
		} else {
			child = cl
		}

		if temp >= a[child] {
			break
		}
		a[parent] = a[child]
	}
	a[parent] = temp
}

func Heapsort(a []int) {
	for i := (len(a) - 1) / 2; i >= 0; i-- {
		downheap(a, i, len(a)-1)
	}
	for i := len(a) - 1; i > 0; i-- {
		a[0], a[i] = a[i], a[0]
		downheap(a, 0, i-1)
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

	// arr := Mergesort(a)
	Heapsort(a)

	fmt.Printf("%v\n", a)
}
