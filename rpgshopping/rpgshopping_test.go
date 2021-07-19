package rpgshopping_test

import (
	"testing"

	"github.com/hsmtkk/animated-couscous/rpgshopping"
	"github.com/stretchr/testify/assert"
)

func TestShopping(t *testing.T) {
	prices := []int{10, 100, 50}
	amount := 300
	buy := []rpgshopping.BuyItem{
		{1, 5},
		{2, 3},
		{3, 1},
		{2, 1},
		{1, 3},
	}
	want := 70
	got := rpgshopping.Solve(prices, amount, buy)
	assert.Equal(t, want, got)
}
