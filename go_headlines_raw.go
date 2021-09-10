package main
import (
	"fmt"
	"net/http"
	"os"
	"bufio"
	"github.com/PuerkitoBio/goquery"
)

func GetLatestHeadlines(url string, filename string, headlineValue string) (string, error) {
	// Get the HTML, convert to GoQuery document
	resp, _ := http.Get(url)
	doc, _ := goquery.NewDocumentFromReader(resp.Body)
	open_file, open_file_error := os.OpenFile(filename, os.O_RDONLY, 0644)
	titles := ""
	if open_file_error != nil {
		// Loop through headlines
		doc.Find(headlineValue).Each(func(i int, s *goquery.Selection) {
			titles += s.Text() + "\n"
		})
		// Create the file and write the headlines
		open_file, _ = os.OpenFile(filename, os.O_CREATE|os.O_WRONLY, 0664)
		defer open_file.Close()
		open_file.Sync()
		open_file.WriteString(titles)
		return titles, nil
	}
	defer open_file.Close()
	// Loop through headlines, this time checking for duplicates
	new_title_found := true
	reader := bufio.NewReader(open_file)
	doc.Find(headlineValue).Each(func(i int, s *goquery.Selection) {
		line, line_err := reader.ReadBytes('\n')
		for line_err == nil {
			test_string := s.Text() + "\n"
			if test_string == string(line) {
				new_title_found = false
				break
			}
			line, line_err = reader.ReadBytes('\n')
		}
		if new_title_found == true {
			titles += s.Text() + "\n"
		}
		new_title_found = true
		open_file.Seek(0, 0)
	})
	// Open in append mode, write only
	open_file_append, _ := os.OpenFile(filename, os.O_APPEND|os.O_WRONLY, 0644)
	open_file_append.WriteString(titles)

	return titles, nil
}

func main() {
	url := ""
	headlineValue := ""
	filenameToWrite := ""
	// Depending on third argument, use different value for HTML quey
	if os.Args[2] == "HN" {
		url = "https://news.ycombinator.com/"
		headlineValue = ".storylink"
		filenameToWrite = os.Args[1] + " HN"
	} else if os.Args[2] == "FT" {
		url = "https://www.ft.com"
		headlineValue = ".js-teaser-heading-link"
		filenameToWrite = os.Args[1] + " FT"
	} else {
		fmt.Println("Unrecognised value passed as second CLI arg")
		os.Exit(4)
	}

	doc, open_erro := GetWebpage(url)
	newHeadlines, _ := GetLatestHeadlines(url, filenameToWrite, headlineValue)
	fmt.Println("New Headlines: " + "\n" + newHeadlines)
}
