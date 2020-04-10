# candelabre

## <span style='color:red;font-weight:bold'>WARNING: libs still in developement (but working ;-) )</span>

Welcome dear reader! You may ask yourself "where am I?", and I have an answer for you: in candelabre main repository!

## What is candelabre?

Candelabre is a multilib project which aims to enable quick prototyping applications with the following features:

* cross-platform
* multi windows capacity
* OpenGL based (for computer which doesn't support vulkan)
* full rust (no sdl, glfw, gtk or Qt inside)
* fully customizable

The project is divide in several libraries:

* [candelabre-windowing](https://github.com/othelarian/candelabre/tree/master/candelabre-windowing)
* [candelabre-widgets](https://github.com/othelarian/candelabre/tree/master/candelabre-widgets) (NOT USABLE YET)

## Some history

Candelabre is a project I started initially to provide a multi windows capacity for glutin, without relying each time to a boilerplate. With this project, I hoped to have ready-to-use API to quickly set up some windows.

So, the candelabre project is originally a attempt to use OpenGL with the help of [luminance](https://github.com/phaazon/luminance-rs) crate to build applications with GUI (candelabre-widgets) and multiple windows (candelabre-windowing). This is the original idea, and from that it slowly moved to something else...

## Crates

### Candelabre windowing system

This crate helps you create windows for your application. Check the
[doc](https://github.com/othelarian/candelabre/tree/master/candelabre-windowing)
for more info

### Candelabre widgets

This crate isn't usable yet, some work is needed. If you want to participate to
the work on this lib, open an issue! ;-)
