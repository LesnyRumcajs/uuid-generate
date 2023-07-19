This is a fork from [This project](https://github.com/LesnyRumcajs/uuid-generate)
I added the End arument `-e`.
This seems to resolve an isue I had: When generating 1 uuid the output inludes and endline char (\n).

Below folows the original README, but with the End option described
---

![build](https://github.com/LesnyRumcajs/uuid-generate/workflows/build/badge.svg?branch=main)
[![Build Status](http://meritbadge.herokuapp.com/uuid-generate)](https://crates.io/crates/uuid-generate/)

## UUID generator
Simple command-line UUID generator (currently generates only v4s)

# Installation
`cargo install uuid-generate`

# Usage
```
USAGE:
    uuid-generate [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d <delimiter>         Delimiter (if n > 1) [default: ,]
    -n <uuid-count>        Number of UUIDs to generate [default: 1]
    -e <end>               The last characters of the output [default: ]
```

# Sample
```
❯ uuid-generate
3cf40924-18f2-4c1d-be76-1c38810d93c8

❯ uuid-generate -n 2 -d '|'
25353496-39c5-4d8c-a0bf-89008c0719e0|afa2b724-0d55-4931-acbe-45ea0ff83708

❯ uuid-generate -e ";;"
17776eb0-deb8-41a9-a143-721a02e7d29b;;
```
