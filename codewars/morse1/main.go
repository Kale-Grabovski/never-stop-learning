package main

import (
	"fmt"
	"strings"
)

var MORSE_CODE = map[string]string{
	"-":         "T",
	"--":        "M",
	"---":       "O",
	"-----":     "0",
	"----.":     "9",
	"---..":     "8",
	"---...":    ":",
	"--.":       "G",
	"--.-":      "Q",
	"--..":      "Z",
	"--..--":    ",",
	"--...":     "7",
	"-.":        "N",
	"-.-":       "K",
	"-.--":      "Y",
	"-.--.":     "(",
	"-.--.-":    ")",
	"-.-.":      "C",
	"-.-.--":    "!",
	"-.-.-.":    ";",
	"-..":       "D",
	"-..-":      "X",
	"-..-.":     "/",
	"-...":      "B",
	"-...- ":    "=",
	"-....":     "6",
	"-....-":    "-",
	".":         "E",
	".-":        "A",
	".--":       "W",
	".---":      "J",
	".----":     "1",
	".----.":    "'",
	".--.":      "P",
	".--.-.":    "@",
	".-.":       "R",
	".-.-.":     "+",
	".-.-.-":    ".",
	".-..":      "L",
	".-..-.":    "\"",
	".-...":     "&",
	"..":        "I",
	"..-":       "U",
	"..---":     "2",
	"..--.-":    "_",
	"..--..":    "?",
	"..-.":      "F",
	"...":       "S",
	"...-":      "V",
	"...--":     "3",
	"...---...": "SOS",
	"...-..-":   "$",
	"....":      "H",
	"....-":     "4",
	".....":     "5",
}

func DecodeMorse(morseCode string) string {
	var ret []string
	words := strings.Split(strings.TrimSpace(morseCode), "   ")
	for _, w := range words {
		letters := strings.Split(w, " ")
		wo := ""
		for _, w := range letters {
			wo += MORSE_CODE[w]
		}
		ret = append(ret, wo)
	}
	return strings.Join(ret, " ")
}

func main() {
	fmt.Println(DecodeMorse(".... . -.--   .--- ..- -.. ."))
}
