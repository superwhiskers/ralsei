# ralsei documentation

hi. this directory contains the nintendo documentation. if you are wondering why it doesn't look
too good in github's built-in markdown viewer, it is because it makes use of some features only
provided to pandoc's tweaked markdown format, the documentation for which can be found
[here](https://pandoc.org/MANUAL.html#pandocs-markdown). if you would like to view it in at its
best, the instructions for building it can be found below

## dependencies

if you are using nixos or any other system with the nix package manager that also has nix-direnv
setup, you do not need to install these; you should already have the dependencies in your
environment, provided that you allowed direnv to execute the project's `.envrc`

- a latex install
- pandoc
- make

## building

run `make`. that's it. you should find `ralsei.pdf` in the `out/` directory. you can also run `make
clean` to clean up any build files that may have been produced (as of now, this target only removes
the `out/` directory.)
