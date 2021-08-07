package speedviolation

type TimeLocation struct {
	Time     int
	Location int
}

func CalcSpeeds(tls []TimeLocation) []int {
	speeds := []int{}
	time_prev := 0
	location_prev := 0
	for _, tl := range tls {
		time_diff := tl.Time - time_prev
		location_diff := tl.Location - location_prev
		if time_diff == 0 {
			continue
		}
		speed := location_diff / time_diff
		speeds = append(speeds, speed)
		time_prev = tl.Time
		location_prev = tl.Location
	}
	return speeds
}

func IsSpeedViolation(limit int, speeds []int) bool {
	for _, speed := range speeds {
		if speed > limit {
			return true
		}
	}
	return false
}
