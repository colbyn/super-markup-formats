# `super-html-ast`

I have too many HTML/XML related crates throughout my many projects. So moving forward I'll be standardizing on these crates for my HTML processing needs. 

## Features

- Browser grade parsing (way more of a headache than it sounds; one day I'm gonna write my own standards incompliant HTML/XML parser).
- HTML data model / AST.
- Formatting including pretty printing via `tidy` (will phase that out eventually).
- Miscellaneous utilities.
- Various AST traversals / visitor pattern implementations.

# `super-markdown-ast`

Mostly used by `super-html-ast` for super basic and trivial markdown formatting. Not well implemented at this time. 
