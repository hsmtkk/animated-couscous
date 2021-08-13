package secretletter_test

import (
	"testing"

	"github.com/hsmtkk/animated-couscous/secretletter"
	"github.com/stretchr/testify/assert"
)

func Test0(t *testing.T) {
	input := "QEPG"
	want := "MILK"
	got := secretletter.New(4).Decrypt(input)
	assert.Equal(t, want, got)
}

func Test1(t *testing.T) {
	input := "KXKPMQVI"
	want := "RQRITJCB"
	got := secretletter.New(19).Decrypt(input)
	assert.Equal(t, want, got)
}
