package main

import (
	"net/http"
	"os"
)

func main() {
	// conn, err := net.Dial("tcp", "ascii.jp:80")
	// if err != nil {
	// 	panic(err)
	// }
	// conn.Write([]byte("GET / HTTP/1.0\r\nHost: ascii.jp\r\n\r\n"))
	// io.Copy(os.Stdout, conn)

	request, err := http.NewRequest("GET", "http://ascii.jp", nil)
	if err != nil {
		panic(err)
	}
	request.Header.Set("X-TEST", "ヘッダーも追加できます")
	request.Write(os.Stdout)
}
