package heightcmp

import (
	"github.com/shopspring/decimal"
)

type Operator int

const (
	Great Operator = iota
	Less
)

type Comparison struct {
	Op     Operator
	Height decimal.Decimal
}

type Range struct {
	Lower decimal.Decimal
	Upper decimal.Decimal
}

func Solve(cmps []Comparison) Range {
	lower := decimal.NewFromInt(0)
	upper := decimal.NewFromInt(999)
	for _, cmp := range cmps {
		switch cmp.Op {
		case Great:
			if cmp.Height.GreaterThan(lower) {
				lower = cmp.Height
			}
		case Less:
			if cmp.Height.LessThan(upper) {
				upper = cmp.Height
			}
		}
	}
	return Range{lower, upper}
}
