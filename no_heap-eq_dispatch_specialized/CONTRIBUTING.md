# Presentation anchors in inline comments
Sections of source files are loaded by https://peter-kehl.github.io/embedded_low_level_rust. Instead
of line numbers, that presentation refers to parts of the source files by strings that are present
in the code in inline comments. Those comments serve as delimiters of code sections to present. So
please leave in any Rust comments `//` or /*...*/ or `Cargo.toml` comments `#` containing
`presentation-`.

# Suggest Rewrap for VS Code
See https://stkb.github.io/Rewrap and
https://marketplace.visualstudio.com/items?itemName=stkb.rewrap

# Occasional two spaces at the end of /// doc lines.
See https://stkb.github.io/Rewrap/specs/features/spaces/#at-the-end-of-a-line. That preserves the
line breaks even when you apply Rewrap's formatting by Alt-Q. That is used to keep some Markdown
code links on the same line - otherwise VS Code wouldn't follow the reference (on Ctrl-click).
