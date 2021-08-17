package rabbitcarrot

import "fmt"

type Carrot struct {
	mass  int
	sugar int
}

func NewCarrot(mass, sugar int) Carrot {
	return Carrot{mass, sugar}
}

type Tester interface {
	Test(carrot Carrot) bool
}

type testerImpl struct {
	sugarStd      int
	acceptedError int
}

func NewTester(sugarStd, acceptedError int) Tester {
	return &testerImpl{sugarStd, acceptedError}
}

func (t *testerImpl) Test(carrot Carrot) bool {
	return t.sugarStd-t.acceptedError < carrot.sugar && carrot.sugar < t.sugarStd+t.acceptedError
}

type Selecter interface {
	Select(tester Tester, carrots []Carrot) (int, error)
}

type selecterImpl struct{}

func NewSelecter() Selecter {
	return &selecterImpl{}
}

func (s *selecterImpl) Select(tester Tester, carrots []Carrot) (int, error) {
	max_mass := 0
	max_index := 0
	for i := 0; i < len(carrots); i++ {
		carrot := carrots[i]
		if tester.Test(carrot) {
			if carrot.mass > max_mass {
				max_mass = carrot.mass
				max_index = i
			}
		}
	}
	if max_mass == 0 {
		return 0, fmt.Errorf("not found")
	} else {
		return max_index, nil
	}
}
