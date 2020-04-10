# Candelabre - Windowing system

Hi dear reader! This crate is for you if you want to quickly set up an app with
one or multiple windows, in full rust!

This crate is built in the idea in mind to use it with other crates, mainly
[luminance](https://github.com/phaazon/luminance-rs) or candelabre-widgets.
The idea is to don't have to write some boilerplate around glutin to make it
work, but let you play with the event loop. Just include a CandlSurface, and
the traits you need, and you're good to go! Working with OpenGL? Higher level
with nanovg/nvg? Want to explore something else? Now you don't need to use sdl
or glfw, or get down in the guts with glutin.

## DISCLAIMER

Two or three things to keep in mind.

First, this project was originally wrote to make app on old computers, without
vulkan support, and cross platform, so it use OpenGL as it's the only way to be
fully cross platform and old computer compliant. So no, there will be no vulkan
support, as it isn't the purpose of this projecT.

Second, the main goal is to handle windowing and widgets for the coders, not
everything. The event loop is completely at the coders discretion, because I
believe it's better to just take care of one thing correctly. So
candelabre-windowing take care of the windowing, not the entire app.

At last, this project isn't over, and a lot of improvment is on their way.
Even if it's stable enough to start building real app with (it's already done),
this crate will evolve, mainly for the state part.

## Actively developped

during the creation process of this library some ideas grow about what to do
better. The current version is already stable, but there is one aspect I want
to explore a little bit deeper. The future evolution will focus on the internal
data capacity of the `CandlSurface` and `CandlManager`. You don't need this
features to use the library, just keep in mind that I work on them.

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
open an issue ;-) .
