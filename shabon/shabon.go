package shabon

type Wind struct {
	x int
	y int
}

func NewWind(x, y int) Wind {
	return Wind{x, y}
}

type Point struct {
	x int
	y int
}

func NewPoint(x, y int) Point {
	return Point{x, y}
}

type Shaboner interface {
	Blow(winds []Wind) int
}

type shabonerImpl struct {
	startPoint Point
}

func New(startPoint Point) Shaboner {
	return &shabonerImpl{startPoint}
}

func (s *shabonerImpl) Blow(winds []Wind) int {
	currentPoint := s.startPoint
	xMax := s.startPoint.x
	for _, wind := range winds {
		if currentPoint.y < 0 {
			break
		}
		currentPoint.x += wind.x
		currentPoint.y += wind.y
		if currentPoint.x > xMax {
			xMax = currentPoint.x
		}
	}
	return xMax
}
