package niceshopping

type Category int

const (
	Food Category = iota
	Book
	Clothing
	Misc
)

type Item struct {
	Category Category
	Price    int
}

func Solve(items []Item) int {
	points := 0
	for _, item := range items {
		switch item.Category {
		case Food:
			points += item.Price * 5 / 100
			break
		case Book:
			points += item.Price * 3 / 100
			break
		case Clothing:
			points += item.Price * 2 / 100
			break
		case Misc:
			points += item.Price / 100
			break
		}
	}
	return points
}
