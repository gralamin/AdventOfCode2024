# AdventOfCode2024
Advent of Code for 2024 - https://adventofcode.com/2023

## Creating a new date executable

Per day, remember to:
```
cd $(git rev-parse --show-toplevel)
git branch --set-upstream-to=origin/main
git pull --rebase
export day=day18
cargo new $day
cp -r template/* $day/
find $day -type f -exec sed -i "s/template/$day/g" {} +
make format
find . -iname "template.day" -delete
git add $day
git commit -m "$day: Added template"
git push origin HEAD:$day
git branch --set-upstream-to origin/$day
git branch -m $day
```

By convention for this repo, so I can ignore it, all programs will be called `<foldername>.day` eg `day01.day`.

To format code, call:

```
make format
```

## Dependencies

To make a new lib:

```
cargo new --lib foo
```

Then you can refer to that lib in the Cargo.toml:

```
[dependencies.my_lib]
path = "../my_lib"
```

And in the code use
```
extern crate my_lib;
```

*Note*: Libs use a slightly different Makefile (no copy)

## Lib list

* `filelib` - A library for common file operations needed in advent of code. Most notably `load_as_ints`, which is used to load input that is just numbers per line.
* `mathlib` - Math operations and functions I might need later.
* `gridlib` - Represents a grid, a common pattern.


# Copyright of Advent of Code
It has been asked to not include inputs, or puzzle texts in this repo. This is the command to clean up this information.

```
git filter-branch -f —tree-filter 'rm -rf day*/input' HEAD
git filter-branch -f —tree-filter 'rm -rf day*/README.md' HEAD
```