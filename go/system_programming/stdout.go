package main

import "os"

func main() {
	os.Stdout.Write([]byte("os.Stdout example\n"))

	// same
	// fmt.Println("os.Stdout example\n")
}
