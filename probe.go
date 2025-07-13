package main

type Frame struct {
	Key   string
	Bytes uint32
}

type File struct {
	Path string
	Data []Frame
}

func NewFile(path string) File {
	return File{
		Path: path,
		Data: []Frame{},
	}
}
