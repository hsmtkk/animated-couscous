package secretletter

import "strings"

type Decryptor interface {
	Decrypt(encrypted string) string
}

type decryptorImpl struct {
	shift int
}

func New(shift int) Decryptor {
	return &decryptorImpl{shift}
}

func (d *decryptorImpl) Decrypt(encrypted string) string {
	decryptedBytes := []byte{}
	for i := 0; i < len(encrypted); i++ {
		if i%2 == 0 {
			decryptedBytes = append(decryptedBytes, d.backward(encrypted[i]))
		} else {
			decryptedBytes = append(decryptedBytes, d.forward(encrypted[i]))
		}
	}
	return string(decryptedBytes)
}

const alphas = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"

func (d *decryptorImpl) forward(b byte) byte {
	idx := strings.Index(alphas, string(b))
	idx = (idx + d.shift) % 26
	return alphas[idx]
}

func (d *decryptorImpl) backward(b byte) byte {
	idx := strings.Index(alphas, string(b))
	idx -= d.shift
	if idx < 0 {
		idx += 26
	}
	return alphas[idx]
}
