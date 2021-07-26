package main

import "fmt"

type Answer struct {
	m int
	n int
	p int
}

func solve() []Answer {
	answers := []Answer{}
	for m := 1; m < 30; m++ {
		for n := 1; n < 30; n++ {
			p2 := m*m + n*n
			ok, p := isSquareNumber(p2)
			if ok {
				answers = append(answers, Answer{m, n, p})
			}
		}
	}
	return answers
}

func isSquareNumber(p2 int) (bool, int) {
	for i := 1; i < p2; i++ {
		if p2 == i*i {
			return true, i
		}
	}
	return false, 0
}

func main() {
	answers := solve()
	for _, ans := range answers {
		fmt.Printf("%v\n", ans)
	}
}
