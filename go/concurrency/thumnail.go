package main

import (
	"log"
	"os"
	"sync"
)

func makeThumnails(filenames <-chan string) int64 {
	seies := make(chan int64)
	var wg sync.WaitGroup
	for f := range filenames {
		wg.Add(1)
		go func(f string) {
			defer wg.Done()
			thumb, err := thumbnail.ImageFile(f)
			if err != nil {
				log.Println(err)
				return
			}
			info, _ := os.Stat(thumb)
			sizes <- info.Size()
		}(f)
	}

	go func() {
		wg.Wait()
		close(sizes)
	}()

	var total int64
	for size := range sizes {
		total += size
	}
	return total
}
