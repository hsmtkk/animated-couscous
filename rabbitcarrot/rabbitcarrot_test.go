package rabbitcarrot_test

import (
	"testing"

	"github.com/hsmtkk/animated-couscous/rabbitcarrot"
	"github.com/stretchr/testify/assert"
)

func TestTest(t *testing.T) {
	tester := rabbitcarrot.NewTester(5, 2)
	assert.False(t, tester.Test(rabbitcarrot.NewCarrot(8, 10)))
	assert.True(t, tester.Test(rabbitcarrot.NewCarrot(7, 6)))
	assert.True(t, tester.Test(rabbitcarrot.NewCarrot(7, 4)))
}

func TestSelect0(t *testing.T) {
	tester := rabbitcarrot.NewTester(5, 2)
	carrots := []rabbitcarrot.Carrot{
		rabbitcarrot.NewCarrot(8, 10),
		rabbitcarrot.NewCarrot(7, 6),
		rabbitcarrot.NewCarrot(7, 4),
	}
	selecter := rabbitcarrot.NewSelecter()
	want := 1
	got, err := selecter.Select(tester, carrots)
	assert.Nil(t, err)
	assert.Equal(t, want, got)
}

func TestSelect1(t *testing.T) {
	tester := rabbitcarrot.NewTester(64, 10)
	carrots := []rabbitcarrot.Carrot{
		rabbitcarrot.NewCarrot(84, 75),
		rabbitcarrot.NewCarrot(73, 53),
		rabbitcarrot.NewCarrot(56, 34),
	}
	selecter := rabbitcarrot.NewSelecter()
	want := 0
	got, err := selecter.Select(tester, carrots)
	assert.Error(t, err)
	assert.Equal(t, want, got)
}
