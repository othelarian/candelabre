# Candelabre - Windowing system

Hi dear reader, this crate is for you if you want to be able to use
[luminance](https://github.com/phaazon/luminance-rs) or candelabre-widgets, by
giving you a simple way to spawn one or multiple windows! For each window, an
OpenGL context is waiting for your next luminous idea to suit it up.

## The CandlSurface

The `CandlSurface` is the first core element in this crate. It allow the
creation of a window with specific parameters. Give it a mode (Windowed or
fullscreen), optionally a size, and a cursor mode (visible or hide), and
we're good to go!

Check the
[candelabre examples](https://github.com/othelarian/candelabre/tree/master/candelabre-examples)
for more details about the way to use it.

## The CandlManager

When you need multiple windows for your application, you need multiple OpenGL
contexts, but managing them is tedious and error prone. To simplify this task,
the `CandlManager` make it easy to swap between OpenGL contexts.

You can find a example of the `CandlManager` in the
[candelabre examples](https://github.com/othelarian/candelabre/tree/master/candelabre-examples).

NOTE: You can't create a window and add it to the manager after, because of
some internal checks. You must choose if you're going multi or not, but don't
panic, moving from one to another is quite easy, and if it's still too obscure,
open an issue.

## TODOLIST

* integration tests
* quickstart example
