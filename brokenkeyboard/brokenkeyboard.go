package brokenkeyboard

type Inputter interface {
	Input(s string) string
}

type inputterImpl struct {
	charLimit map[byte]int
	charCount map[byte]int
}

func New(limits []int) Inputter {
	charLimit := map[byte]int{}
	charCount := map[byte]int{}
	alphas := "abcdefghijklmnopqrstuvwxyz"
	for i := 0; i < 26; i++ {
		charLimit[alphas[i]] = limits[i]
		charCount[alphas[i]] = 0
	}
	return &inputterImpl{charLimit, charCount}
}

func (i *inputterImpl) Input(s string) string {
	results := []byte{}
	for _, b := range []byte(s) {
		limit := i.charLimit[b]
		count := i.charCount[b]
		if count < limit {
			results = append(results, b)
		}
		i.charCount[b] = count + 1
	}
	return string(results)
}
