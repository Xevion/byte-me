package main

import (
	"context"
	"fmt"

	"github.com/wailsapp/wails/v2/pkg/runtime"
	"gopkg.in/vansante/go-ffprobe.v2"
)

// App struct
type App struct {
	ctx   context.Context
	files []File
}

// NewApp creates a new App application struct
func NewApp() *App {
	return &App{
		files: []File{},
	}
}

// startup is called when the app starts. The context is saved
// so we can call the runtime methods
func (a *App) startup(ctx context.Context) {
	runtime.OnFileDrop(ctx, func(x, y int, paths []string) {
		a.OnFileDrop(x, y, paths)
	})

	a.ctx = ctx
}

func (a *App) shutdown(ctx context.Context) {
	runtime.OnFileDropOff(ctx)
}

func (a *App) GetStream(path string) {
}

func (a *App) OnFileDrop(x, y int, paths []string) {
	runtime.LogPrint(a.ctx, fmt.Sprintf("OnFileDrop: %v", paths))

	for _, path := range paths {
		probe, err := ffprobe.ProbeURL(a.ctx, path)
		if err != nil {
			runtime.LogPrint(a.ctx, fmt.Sprintf("Error: %v", err))
			continue
		}

		for _, stream := range probe.Streams {
			runtime.LogPrint(a.ctx, fmt.Sprintf("Stream: %v", stream))
		}
	}
}

// Greet returns a greeting for the given name
func (a *App) Greet(name string) string {
	return fmt.Sprintf("Hello %s, It's show time!", name)
}
