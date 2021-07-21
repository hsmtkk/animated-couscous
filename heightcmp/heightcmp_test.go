package heightcmp_test

import (
	"log"
	"testing"

	"github.com/hsmtkk/animated-couscous/heightcmp"
	"github.com/shopspring/decimal"
	"github.com/stretchr/testify/assert"
)

func TestSolve(t *testing.T) {
	inputs := []heightcmp.Comparison{
		{heightcmp.Less, dec("120.3")},
		{heightcmp.Great, dec("115.7")},
		{heightcmp.Less, dec("122.0")},
		{heightcmp.Great, dec("116.9")},
		{heightcmp.Less, dec("119.1")},
		{heightcmp.Less, dec("117.6")},
	}
	want := heightcmp.Range{dec("116.9"), dec("117.6")}
	got := heightcmp.Solve(inputs)
	assert.Equal(t, want, got)
}

func dec(f string) decimal.Decimal {
	d, err := decimal.NewFromString(f)
	if err != nil {
		log.Fatal(err)
	}
	return d
}
