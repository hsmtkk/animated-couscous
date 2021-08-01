package janken

type Te int

const (
	Paper Te = iota
	Rock
	Scissors
	Draw
)

type Solver interface {
	Add(te Te)
	Solve() Te
}

type solverImpl struct {
	tes map[Te]bool
}

func New() Solver {
	tes := map[Te]bool{}
	tes[Paper] = false
	tes[Rock] = false
	tes[Scissors] = false
	return &solverImpl{tes}
}

func (s *solverImpl) Add(te Te) {
	s.tes[te] = true
}

func (sv *solverImpl) Solve() Te {
	p := sv.tes[Paper]
	r := sv.tes[Rock]
	s := sv.tes[Scissors]
	if p && r && s {
		return Draw
	} else if p && r {
		return Paper
	} else if p && s {
		return Scissors
	} else if r && s {
		return Rock
	} else {
		return Draw
	}
}
