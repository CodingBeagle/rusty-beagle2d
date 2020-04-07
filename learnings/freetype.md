# FreeType

FreeType is a font engine.

It provides an API for accessing and using font content in a uniform way, independent of the underlying filetype.

## Typographic Concepts

- Font Family
  - A "template" for a specific style of font.
  - A collection of *font faces*.
- Font Face
  - A stylistic instance of a font family.
  - Example: "Palatino Regular" and "Palatino Italic" are two *font faces* belonging to the font family "Palatino".
- Digital Font
  - A data file consisting of one or more font faces, having important information about the font faces such as character metrics, character images, etc...

## Character Images

Character images are called **glyphs**.

A single character can have several distinct images (several glyphs).

- A font file contains a set of glyphs.
  - Each glyph can be stored as a bitmap, a vector representation, or any other scheme.
    - Most scalable formats use a combination of mathematical representation and control data).
  - Glyphs can be stored in any order within the font file.
    - They are typically accessed be using a simple glyph index.
- The font file also contains one or more *character maps*.
  - These are used to convert character codes for an encoding (like ASCII, Unicode, etc...) into glyph indices relative to the font file.

## Character and font metrics

Each *glyph image* is associated with metrics that describe how to place and manage it when rendering text.

These metrics can be things like: glyph placement, cursor advances, text layout. They are **crucial** to compute the flow of text when rendering a string of text.

### Baseline, pens, and layouts

- Baseline
  - An imaginary line that is used to guide glyphs when rendering text.
  - The baseline can be horizontal or vertical.
- Pen Position
  - A virtual point, located on the baseline.

*To render text, we increment the pen position*. The distance between two successive pen positions is the **advance width**, and is specific to each individual glyph image.