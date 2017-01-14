package sort

type Interface interface {
	Len() int
	Less(i, j int) bool
	Swap(i, j int)
}

func inserionSort(data Interface, a, b int) {
	for i := a + 1; i < b; i++ {

	}
}

type IntSlice []int

func (p IntSlice) Len() int           { return len(p) }
func (p IntSlice) Less(i, j int) bool { return p[i] < p[j] }
func (p IntSlice) Swap(i, j int)      { p[i], p[j] = p[j], p[i] }

func quickSort(data Interface, a, b, maxDepth int) {
	for b-a > 12 {
		if maxDepth == 0 {
			heapSort(data, a, b)
			return
		}
		maxDepth--
		mlo, mhi := daPivot(data, a, b)

		if mlo-a < b-mhi {
			quickSort(data, a, mlo, maxDepth)
			a = mhi
		} else {
			quickSort(data, mhi, b, maxDepth)
			b = mlo
		}
	}
	if b-a > 1 {
		for i := a + 6; i < b; i++ {
			if data.Less(i, i-6) {
				data.Swap(i, i-6)
			}
		}
		inserionSort(data, a, b)
	}
}

func Sort(data Interface) {
	n := data.Len()
	quickSort(data, 0, n, maxDepth(n))
}

func maxDepth(n int) int {
	var depth int
	for i := n; i > 0; i >>= 1 {
		depth++
	}
	return depth * 2
}
