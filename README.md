# Interactive HTML BOM Generator

[Documentation](https://docs.rs/interactive-html-bom/)

A Rust library implementing a subset of
[InteractiveHtmlBom](https://github.com/openscopeproject/InteractiveHtmlBom),
to generate Bill of Materials for PCBs as an interactive HTML page.
See their project description for details, and check out the example output
[here](https://openscopeproject.org/InteractiveHtmlBomDemo/).

This library does not re-implement the complete functionality of the upstream
project. Instead, it re-uses their HTML/CSS/JS files and provides a minimal
high-level API to perform just the HTML generation. Differences to the
upstream project:

- Rust instead of Python
- No CLI, just a Rust library
- Not a plugin for EDA tools
- No parser for EDA project files
- Stripped down support of the
  [`pcbdata`](https://github.com/openscopeproject/InteractiveHtmlBom/blob/f9a419b2b19bcb86dd81c61f0b7feba8dffce9f4/DATAFORMAT.md) structure, especially
  missing support for any geometry type other than `polygon` with `svgdata`
  since this type is enough to draw any kind of shape

The library has been developed for integration in
[LibrePCB](https://librepcb.org/), though it's API is generic and thus would
be usable for other projects too.

## License and Credits

Library is licensed under MIT license, see [`LICENSE`](./LICENSE) for details.

A huge thanks to the developers of
[InteractiveHtmlBom](https://github.com/openscopeproject/InteractiveHtmlBom)
who created the awesome HTML/CSS/JS which we're reusing in this project.

The implementation of this library was funded by the
[NGI Zero Commons Fund](https://nlnet.nl/commonsfund/) as part of the
[LibrePCB 2.0](https://nlnet.nl/project/LibrePCB2.0/) project.
