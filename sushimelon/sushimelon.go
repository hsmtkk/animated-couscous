package sushimelon

type Solver interface {
	Solve(netas []string) int
}

type solverImpl struct{}

func New() Solver {
	return &solverImpl{}
}

func (s *solverImpl) Solve(netas []string) int {
	melon := 0
	count := 0
	for _, neta := range netas {
		count -= 1
		if count > 0 {
			continue
		}
		if neta == "melon" {
			melon += 1
			count = 10
		}
	}
	return melon
}
