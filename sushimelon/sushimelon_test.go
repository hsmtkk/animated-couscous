package sushimelon_test

import (
	"testing"

	"github.com/hsmtkk/animated-couscous/sushimelon"
	"github.com/stretchr/testify/assert"
)

func TestSolve(t *testing.T) {
	netas := []string{
		"melon",
		"ikura",
		"sake",
		"kappa",
		"maguro",
		"melon",
		"uni",
		"toro",
		"ebi",
		"amaebi",
		"tamagoyaki",
		"ika",
		"anago",
		"melon",
		"hamachi",
	}
	solver := sushimelon.New()
	got := solver.Solve(netas)
	want := 2
	assert.Equal(t, want, got)
}
