package orangecategory_test

import (
	"testing"

	"github.com/hsmtkk/animated-couscous/orangecategory"
	"github.com/stretchr/testify/assert"
)

func Test0(t *testing.T) {
	cat := orangecategory.New(10)
	assert.Equal(t, 20, cat.Categorize(24))
	assert.Equal(t, 40, cat.Categorize(35))
	assert.Equal(t, 10, cat.Categorize(3))
}

func Test1(t *testing.T) {
	cat := orangecategory.New(50)
	assert.Equal(t, 50, cat.Categorize(40))
	assert.Equal(t, 100, cat.Categorize(90))
	assert.Equal(t, 50, cat.Categorize(10))
}

func Test2(t *testing.T) {
	cat := orangecategory.New(3)
	assert.Equal(t, 9, cat.Categorize(9))
	assert.Equal(t, 6, cat.Categorize(5))
}
