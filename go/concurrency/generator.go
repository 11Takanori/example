package main

import (
	"fmt"
	"math/rand"
)

func main() {
	repeatFn := func(
		done <-chan interface{},
		fn func() interface{},
	) <-chan interface{} {
		valueStream := make(chan interface{})
		go func() {
			for {
				select {
				case <-done:
					return
				case valueStream <- fn():
				}
			}
		}()
		return valueStream
	}

	done := make(chan interface{})
	defer close(done)

	rand := func() interface{} { return rand.Int() }

	for num := range take(done, repeatFn(done, rand), 10) {
		fmt.Printf("%v ", num)
	}
}
