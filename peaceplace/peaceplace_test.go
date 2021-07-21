package peaceplace_test

import (
	"testing"

	"github.com/hsmtkk/animated-couscous/peaceplace"
	"github.com/stretchr/testify/assert"
)

func TestSolve(t *testing.T) {
	kouji := peaceplace.Kouji{X: 20, Y: 10, R: 10}
	wants := map[peaceplace.Point]bool{}
	wants[peaceplace.Point{25, 10}] = true
	wants[peaceplace.Point{20, 15}] = true
	wants[peaceplace.Point{70, 70}] = false
	for want_point, want_noisy := range wants {
		got := kouji.Noisy(want_point)
		assert.Equal(t, want_noisy, got)
	}
}
