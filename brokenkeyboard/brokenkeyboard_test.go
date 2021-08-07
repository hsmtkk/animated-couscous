package brokenkeyboard_test

import (
	"testing"

	"github.com/hsmtkk/animated-couscous/brokenkeyboard"
	"github.com/stretchr/testify/assert"
)

func Test0(t *testing.T) {
	limits := []int{1, 3, 2, 6, 3, 5, 5, 6, 2, 6, 0, 4, 5, 2, 4, 2, 1, 2, 4, 0, 4, 2, 2, 5, 0, 2}
	inputter := brokenkeyboard.New(limits)
	want := "abcbcb"
	got := inputter.Input("abcabcabcabc")
	assert.Equal(t, want, got)
}
