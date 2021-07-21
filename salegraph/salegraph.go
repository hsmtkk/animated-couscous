package salegraph

import "strings"

type Drawer interface {
	Draw(sale int) string
}

type drawerImpl struct {
	div   int
	width int
}

func New(div, width int) Drawer {
	return &drawerImpl{div, width}
}

func (d *drawerImpl) Draw(sale int) string {
	marks := sale / d.div
	return strings.Repeat("*", marks) + strings.Repeat(".", d.width-marks)
}
