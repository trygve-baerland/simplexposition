# Simplexposition

## Documentation

The documentation is built as a book, using
the [mdbook](https://rust-lang.github.io/mdBook/) tool.

So it's necessary to install this,
as well as some additional plugins:

```bash
cargo install mdbook
cargo install mdbook-bib
```

You can build the book using

```bash
make docs-build
```

and serve it by

```bash
make docs-serve
```