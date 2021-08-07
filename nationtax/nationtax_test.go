package nationtax_test

import (
	"testing"

	"github.com/hsmtkk/animated-couscous/nationtax"
	"github.com/stretchr/testify/assert"
)

func Test0(t *testing.T) {
	incomes := []int{450000, 200000, 100, 2000000, 1234567}
	taxes := []int{}
	calculator := nationtax.New()
	for _, income := range incomes {
		tax := calculator.CalculateTax(income)
		taxes = append(taxes, tax)
	}
	got := calculator.Sum(taxes)
	want := 621913
	assert.Equal(t, want, got)
}
