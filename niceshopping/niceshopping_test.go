package niceshopping_test

import (
	"testing"

	"github.com/hsmtkk/animated-couscous/niceshopping"
	"github.com/stretchr/testify/assert"
)

func TestShopping(t *testing.T) {
	items := []niceshopping.Item{
		{niceshopping.Food, 200},
		{niceshopping.Book, 240},
		{niceshopping.Food, 120},
		{niceshopping.Misc, 460},
		{niceshopping.Book, 240},
		{niceshopping.Clothing, 3200},
	}
	want := 95
	got := niceshopping.Solve(items)
	assert.Equal(t, want, got)
}
