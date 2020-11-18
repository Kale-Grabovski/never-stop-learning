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

func getSpeed(bits string) int {
	speed := 9999999
	can := 0
	for _, c := range bits {
		if c == '0' {
			can++
		} else {
			if can > 0 && can < speed {
				speed = can
			}
			can = 0
		}
	}
	if can > 0 && can < speed {
		speed = can
	}
	return speed
}

func DecodeBits(bits string) string {
	speed := getSpeed(bits)

	bits = strings.Trim(bits, "0")
	if bits == "" {
		return ""
	}

	if speed == 0 {
		return strings.Repeat(". ", len(bits))
	}

	prev := int32(bits[0])
	c := 0

	shit := func() string {
		switch {
		case prev == '0' && c/speed == 7:
			return "   "
		case prev == '0' && c/speed == 3:
			return " "
		case prev == '1' && c/speed == 3:
			return "-"
		case prev == '1' && c/speed == 1:
			return "."
		}
		return ""
	}

	morse := ""
	for _, b := range bits {
		if b == prev {
			c++
			continue
		}
		morse += shit()
		c = 1
		prev = b
	}
	morse += shit()

	return morse
}

func main() {
	fmt.Println(DecodeMorse(DecodeBits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011")))
	fmt.Println(DecodeMorse(DecodeBits("11")))
	fmt.Println(DecodeMorse(DecodeBits("1")))
}
