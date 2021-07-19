package rpgshopping

type BuyItem struct {
	Index int
	Count int
}

func Solve(prices []int, amount int, buyItems []BuyItem) int {
	for _, b := range buyItems {
		idx := b.Index - 1
		price := prices[idx] * b.Count
		if amount > price {
			amount -= price
		}
	}
	return amount
}
