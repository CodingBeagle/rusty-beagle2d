# Rusty Beagle 2D
Welcome to Rusty Beagle 2D! A (to be) 2D renderer and 2D game engine.

This is a hobby project of mine, with the purpose of learning the programming language [Rust](https://www.rust-lang.org/) whilst also learning real-time rendering.

## Current Status
Currently the project is still in very early stages, as I'm still learning to navigate Rust. A large chunk of the initial development time has been spent learning the Rust FFI journey, in order to interface with C libraries (such as OpenGL, GLFW, etc...).

At the current stage I have successfully gotten to the classic *Hello World* stage of rendering (well, perhaps a bit more...), in that I can render a 2D sprite/texture. I can also transform the sprite (rotate, translate, and scale). 

Here's a screenshot of me rendering a rotated and scaled beagle...

![screenshot](https://gitlab.com/CodingBeagle/rusty-beagle2d/-/raw/master/rusty-beagle.png)

# GLFW and OpenGL Crates
As part of the project I'm creating higher level wrappers around the low-level FFI functions to the libraries I use. So far, this is OpenGL and GLFW.

I wouldn't recommend that anyone use them, as they are being refactored and otherwise changed constantly, as I learn more about the language and improve.
