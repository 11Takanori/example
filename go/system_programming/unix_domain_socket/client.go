package main

import (
	"net"
)

func main() {
	_, err := net.Dial("unix", "socketfile")
	if err != nil {
		panic(err)
	}
}
