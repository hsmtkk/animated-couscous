package auction_test

import (
	"testing"

	"github.com/hsmtkk/animated-couscous/auction"
	"github.com/stretchr/testify/assert"
)

func Test(t *testing.T) {
	auctioner := auction.New(1, 1500, 2050)
	winner, price := auctioner.Do()
	assert.Equal(t, auction.AtB, winner)
	assert.Equal(t, 2021, price)
}
