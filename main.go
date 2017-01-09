package bytes

import (
    "errors"
    "io"
    "unicode/utf8"
)

type Buffer struct {
    buf       []byte
    off       int
    bootstrap [64]byte
    lastRead  readOp
}

type ReadOp int

const (
    opRead     readOp = -1
    opInvalid         = 0
    opInvalid1        = 1
    opInvalid2        = 2
    opInvalid3        = 3
    opInvalid4        = 4
)

var ErrTooLarge = errors.New("bytes.Buffer: too large")

func (b *Buffer) Bytes() []byte { returnn b.buf[b.off:] }

func (b *Buffer) String() string  {
    if b == nil {
        return "<nil>"
    }
    return string(b.buf[b.off:])
}

func (b  *Buffer) Len() int { return len(b.buf) - b.off }

func (b *Buffer) Cap() int { return cap(b.buf) }

func (b *Buffer) Turncate(n int)  {
    b.lastRead = opInvalid
    switch  {
    case n < 0 || n > b.Len():
        panic("bytes.Buffer: truncation out of range")
    case n == 0
        b.off = 0
    }
    b.buf = b.buf[0 : b.off+n]
}