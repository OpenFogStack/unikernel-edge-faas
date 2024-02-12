package main

import (
	"archive/zip"
	"fmt"
	"gonum.org/v1/gonum/mat"
	"io"
	"io/ioutil"
	"math/rand"
	"net/http"
	"os"
	"path/filepath"
	"runtime"
	"scientificgo.org/fft"
	"strconv"
	"time"
)

func readyCallback() {
	if len(os.Args) <= 1 {
		fmt.Printf("Missing callback. Exiting\n")
		os.Exit(1)
	}

	requestURL := os.Args[1]

	if requestURL == "ignore" {
		fmt.Printf("Ignoring callback. Continuing\n")
		return
	}

	fmt.Printf("Calling callback at %s\n", requestURL)
	res, err := http.Get(requestURL)
	if err != nil {
		fmt.Printf("error making http request: %s\n", err)
		os.Exit(1)
	}

	fmt.Printf("client: got response!\n")
	fmt.Printf("client: status code: %d\n", res.StatusCode)
}

func helloHandler(w http.ResponseWriter, r *http.Request) {
	// fmt.Printf("got /hello request\n")
	io.WriteString(w, "Hello world from Go\n")
}

func fibHandler(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("n")
	n, err := strconv.ParseUint(param, 10, 64)
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		io.WriteString(w, "Invalid parameter")
		return
	}

	var a, b uint64

	if n < 2 {
		b = n
	} else {
		b = 1
		for n--; n > 0; n-- {
			a += b
			a, b = b, a
		}
	}

	io.WriteString(w, fmt.Sprintf("Done (%d)\n", b))
}

func fftHandler(w http.ResponseWriter, r *http.Request) {
	n := r.URL.Query().Get("n")
	size, err := strconv.Atoi(n)
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		io.WriteString(w, "Invalid parameter")
		return
	}

	data := make([]complex128, size)

	for i := 0; i < size; i++ {
		real := rand.Float64()
		imaginary := rand.Float64()
		data[i] = complex(real, imaginary)
	}

	start := time.Now()
	fft.Fft(data, false)
	elapsed := time.Since(start)

	io.WriteString(w, fmt.Sprintf("Done (%d)\n", elapsed.Milliseconds()))
}

func matrixHandler(w http.ResponseWriter, r *http.Request) {
	n := r.URL.Query().Get("n")
	size, err := strconv.Atoi(n)
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		io.WriteString(w, "Invalid parameter")
		return
	}

	data := make([]float64, size*size)
	for i := range data {
		data[i] = rand.NormFloat64()
	}
	a := mat.NewDense(size, size, data)

	start := time.Now()
	a.Mul(a, a)
	elapsed := time.Since(start)

	io.WriteString(w, fmt.Sprintf("Done (%d)\n", elapsed.Milliseconds()))
}

var readPath string = "./static_file"

func readHandler(w http.ResponseWriter, r *http.Request) {
	file, err := os.Open(readPath)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		e := fmt.Sprintln("Error opening file:", err)
		io.WriteString(w, e)
		return
	}
	defer file.Close()

	fileContent, err := ioutil.ReadAll(file)
	if err != nil {
		e := fmt.Sprintln("Error reading file:", err)
		io.WriteString(w, e)
		return
	}

	io.WriteString(w, fmt.Sprintf("Done (read %d bytes)\n", len(fileContent)))
}

func allocHandler(w http.ResponseWriter, r *http.Request) {
	size := 50 * 1024 * 1024
	data := make([]uint8, size)
	// cause a page fault for every allocated page
	for i := 0; i < size; i += 4096 {
		data[i] = 0xff
	}
}

func unzipHandler(w http.ResponseWriter, r *http.Request) {
	zipReader, err := zip.OpenReader(readPath)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		e := fmt.Sprintln(err)
		io.WriteString(w, e)
		return
	}
	defer zipReader.Close()

	for _, file := range zipReader.Reader.File {
		zippedFile, err := file.Open()
		if err != nil {
			w.WriteHeader(http.StatusInternalServerError)
			e := fmt.Sprintln(err)
			io.WriteString(w, e)
			return
		}
		defer zippedFile.Close()

		targetDir := "./unzipped"
		extractedFilePath := filepath.Join(
			targetDir,
			file.Name,
		)

		if file.FileInfo().IsDir() {
			fmt.Println("directory:", extractedFilePath)
			os.MkdirAll(extractedFilePath, file.Mode())
		} else {
			fmt.Println("file:", file.Name)

			flags := os.O_WRONLY | os.O_CREATE | os.O_TRUNC
			outputFile, err := os.OpenFile(
				extractedFilePath,
				flags,
				file.Mode(),
			)

			if err != nil {
				w.WriteHeader(http.StatusInternalServerError)
				e := fmt.Sprintln(err)
				io.WriteString(w, e)
				return
			}

			defer outputFile.Close()

			_, err = io.Copy(outputFile, zippedFile)
			if err != nil {
				w.WriteHeader(http.StatusInternalServerError)
				e := fmt.Sprintln(err)
				io.WriteString(w, e)
				return
			}
		}
	}

	io.WriteString(w, "Done")
}

func main() {
	fmt.Printf("Go version: %s, listening on port 8080 ...\n", runtime.Version())
	fmt.Println("Reading static file from", readPath)
	http.HandleFunc("/hello", helloHandler)
	http.HandleFunc("/fib", fibHandler)
	http.HandleFunc("/fft", fftHandler)
	http.HandleFunc("/matrix", matrixHandler)
	http.HandleFunc("/read", readHandler)
	http.HandleFunc("/unzip", unzipHandler)
	http.HandleFunc("/alloc", allocHandler)
	http.HandleFunc("/static", func(w http.ResponseWriter, r *http.Request) {
		http.ServeFile(w, r, readPath)
	})
	go readyCallback()
	http.ListenAndServe("0.0.0.0:8080", nil)
}
