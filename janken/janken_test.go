package janken_test

import (
	"testing"

	"github.com/hsmtkk/animated-couscous/janken"
	"github.com/stretchr/testify/assert"
)

func Test0(t *testing.T) {
	solver := janken.New()
	solver.Add(janken.Paper)
	solver.Add(janken.Paper)
	solver.Add(janken.Rock)
	assert.Equal(t, janken.Paper, solver.Solve())
}

func Test1(t *testing.T) {
	solver := janken.New()
	solver.Add(janken.Rock)
	solver.Add(janken.Paper)
	solver.Add(janken.Rock)
	solver.Add(janken.Scissors)
	assert.Equal(t, janken.Draw, solver.Solve())
}
