package speedviolation_test

import (
	"testing"

	"github.com/hsmtkk/animated-couscous/speedviolation"
	"github.com/stretchr/testify/assert"
)

func Test0(t *testing.T) {
	inputs := []speedviolation.TimeLocation{
		{0, 0},
		{1, 30},
		{2, 80},
	}
	want := []int{30, 50}
	speeds := speedviolation.CalcSpeeds(inputs)
	assert.Equal(t, want, speeds)
	assert.True(t, speedviolation.IsSpeedViolation(40, speeds))
}

func Test1(t *testing.T) {
	inputs := []speedviolation.TimeLocation{
		{1, 50},
		{2, 100},
		{3, 150},
		{5, 200},
		{8, 250},
	}
	want := []int{50, 50, 50, 25, 16}
	speeds := speedviolation.CalcSpeeds(inputs)
	assert.Equal(t, want, speeds)
	assert.True(t, speedviolation.IsSpeedViolation(40, speeds))
}
