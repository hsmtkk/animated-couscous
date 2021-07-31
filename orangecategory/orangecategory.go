package orangecategory

import (
	"math"
)

type Categorizer interface {
	Categorize(weight int) int
}

func New(boxWeight int) Categorizer {
	return &categorizerImpl{boxWeight}
}

type categorizerImpl struct {
	boxWeight int
}

func (c *categorizerImpl) Categorize(weight int) int {
	x := float64(weight) / float64(c.boxWeight)
	n := int(math.Round(x))
	if n == 0 {
		return c.boxWeight
	} else {
		return n * c.boxWeight
	}
}
