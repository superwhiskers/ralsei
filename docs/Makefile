# ralsei - fast nintendo library in rust
#
# copyright (c) 2020-2021 superwhiskers <whiskerdev@protonmail.com>
# this source code form is subject to the terms of the mozilla public
# license, v. 2.0. if a copy of the mpl was not distributed with this
# file, you can obtain one at http://mozilla.org/MPL/2.0/.

# makefile prelude

ifeq ($(origin .RECIPEPREFIX), undefined)
	$(error your make version does not support .RECIPEPREFIX. please use a newer version of make)
endif

.RECIPEPREFIX = >
.ONESHELL:
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

# general targets

.PHONY: all
all: out/ralsei.pdf

.PHONY: clean
clean:
> @rm -rf out

# file targets

out/ralsei.pdf: src/index.md           \
	src/types.md           \
	src/servers/headers.md \
	src/formats/gfmodel.md
	# src/servers/account.md
> @mkdir -p $(@D)
> @pandoc -f markdown -t pdf --toc -o $@ $^
