# Candelabre - Examples

Hello dear reader!

Here you can find some examples to start working with candelabre!

## Simple window

What can you do with just candelabre-windowing? Well, at least you can open a
window! This is the main goal of this lib, and this example show you the basis,
with a single window, and the event loop. With the utils and some nvg (rust
implementation of nanovg), you have now a colored triangle and a way to change
the state of the window.

## Multi windows

But the true strength of candelabre-windowing is the `CandlManager`, which open
for you the gates of multiple windows in the same app. You think it's standard?
Take a look at some libs, and this isn't as common as you can imagine. This was
the second goal of candelabre-windowing, to avoid boilerplate from glutin
examples to handle OpenGL current context switching.

## Luminance

When candelabre-windowing was only a PoC (called nikut), the idea was to use 
[luminance](https://github.com/phaazon/luminance-rs) as the backbone for the
graphics. Luminance is a great crate, take a look if you want to use OpenGL.
In the candelabre context, luminance isn't mandatory, but you can use it, and
this example show you how.
