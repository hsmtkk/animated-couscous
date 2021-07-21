package peaceplace

type Kouji struct {
	X int
	Y int
	R int
}

func (k Kouji) Noisy(p Point) bool {
	return (k.X-p.X)*(k.X-p.X)+(k.Y-p.Y)*(k.Y-p.Y) < k.R*k.R
}

type Point struct {
	X int
	Y int
}
