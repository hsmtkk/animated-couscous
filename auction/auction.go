package auction

type Attendees int

const (
	AtA Attendees = iota
	AtB
)

type Auctioner interface {
	Do() (Attendees, int)
}

type auctionerImpl struct {
	initPrice int
	aLimit    int
	bLimit    int
}

func New(initPrice, aLimit, bLimit int) Auctioner {
	return &auctionerImpl{initPrice, aLimit, bLimit}
}

func (a *auctionerImpl) Do() (Attendees, int) {
	price := a.initPrice
	for i := 0; ; i++ {
		if i%2 == 0 {
			// a turn
			nextPrice := price + 10
			if nextPrice < a.aLimit {
				price = nextPrice
			} else {
				return AtB, price
			}
		} else {
			// b turn
			nextPrice := price + 1000
			if nextPrice < a.bLimit {
				price = nextPrice
			} else {
				return AtA, price
			}
		}
	}
}
