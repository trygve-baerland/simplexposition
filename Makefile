# Build documentation
.PHONY: docs-build
docs-build:
	mdbook build docs

.PHONY: docs-serve
docs-serve:
	mdbook serve docs
