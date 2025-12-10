package main

import "fmt"

type rect2 struct {
	Min, Max Pos2
}

func NewRect2(p1, p2 *Pos2) *rect2 {
	return &rect2{
		Min: Pos2{MinCmp(p1.X, p2.X), MinCmp(p1.Y, p2.Y)},
		Max: Pos2{MaxCmp(p1.X, p2.X), MaxCmp(p1.Y, p2.Y)},
	}
}

func (r *rect2) WH() (int, int) {
	return r.Max.X - r.Min.X + 1, r.Max.Y - r.Min.Y + 1
}

func (r *rect2) IsCorner(p *Pos2) bool {
	return p.Equals(&r.Min) || p.Equals(&r.Max) || p.Equals(&Pos2{r.Min.X, r.Max.Y}) || p.Equals(&Pos2{r.Max.X, r.Min.Y})
}

func (r *rect2) Contains(p *Pos2) bool {
	return p.X >= r.Min.X && p.X <= r.Max.X && p.Y >= r.Min.Y && p.Y <= r.Max.Y
}

func (r *rect2) Area() int {
	return (r.Max.X - r.Min.X + 1) * (r.Max.Y - r.Min.Y + 1)
}

func (r *rect2) String() string {
	return fmt.Sprintf("Rect2{%d,%d}", r.Min, r.Max)
}
