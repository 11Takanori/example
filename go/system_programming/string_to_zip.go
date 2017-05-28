package main

import "archive/zip"
import "os"
import "io"
import "strings"

func main() {
	file, err := os.Create("sample.zip")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	zipWriter := zip.NewWriter(file)
	defer zipWriter.Close()

	x, err := zipWriter.Create("x.txt")
	if err != nil {
		panic(err)
	}
	io.Copy(x, strings.NewReader("x"))

	y, err := zipWriter.Create("y.txt")
	if err != nil {
		panic(err)
	}
	io.Copy(y, strings.NewReader("y"))
}
