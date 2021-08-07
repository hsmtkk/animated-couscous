package nationtax

type Calculator interface {
	CalculateTax(income int) int
	Sum(ns []int) int
}

func New() Calculator {
	return &calculatorImpl{}
}

type calculatorImpl struct{}

func (c *calculatorImpl) CalculateTax(income int) int {
	return int(float32(income) * c.getRate(income))
}

func (c *calculatorImpl) getRate(income int) float32 {
	if 0 <= income && income < 100001 {
		return 0.0
	} else if 100001 <= income && income < 750001 {
		return 0.1
	} else if 750001 <= income && income < 1500001 {
		return 0.2
	} else {
		return 0.4
	}
}

func (c *calculatorImpl) Sum(ns []int) int {
	s := 0
	for _, n := range ns {
		s += n
	}
	return s
}
