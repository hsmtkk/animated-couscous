package shabon_test

import (
	"testing"

	"github.com/hsmtkk/animated-couscous/shabon"
	"github.com/stretchr/testify/assert"
)

func Test0(t *testing.T) {
	shaboner := shabon.New(shabon.NewPoint(1, 1))
	winds := []shabon.Wind{
		shabon.NewWind(4, 2),
		shabon.NewWind(-5, -4),
		shabon.NewWind(3, 3),
		shabon.NewWind(3, 3),
	}
	got := shaboner.Blow(winds)
	want := 5
	assert.Equal(t, want, got)
}
