# Ramiel

A simple SVG drawing tool.

This project is mostly a learning exercise but it's intended to be useful.

* It can only be used to draw a single polygon.
* It only saves to SVG.
* Files can't be opened in it.
* The polygon can't be styled. Using the SVG on the web means you can style it with CSS.

## How to Use

* Left-click to add a vertex.
* Right-click to delete a vertex.
* Ctrl+S to save as SVG.

## Build Dependencies

* `libsdl2-dev`
* `libsdl2-gfx-dev`
* `libsdl2-image-dev`
* possibly `libgtk-3-dev` or something so that `nfd` works. You could try building and read the error message for what's missing.
