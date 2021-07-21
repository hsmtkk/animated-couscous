package salegraph_test

import (
	"testing"

	"github.com/hsmtkk/animated-couscous/salegraph"
	"github.com/stretchr/testify/assert"
)

func Test0(t *testing.T) {
	drawer := salegraph.New(5, 3)
	assert.Equal(t, "*..", drawer.Draw(5))
	assert.Equal(t, "***", drawer.Draw(15))
	assert.Equal(t, "**.", drawer.Draw(10))
}

func Test1(t *testing.T) {
	drawer := salegraph.New(2, 8)
	assert.Equal(t, "******..", drawer.Draw(12))
	assert.Equal(t, "****....", drawer.Draw(8))
	assert.Equal(t, "********", drawer.Draw(16))
	assert.Equal(t, "*****...", drawer.Draw(10))
}
