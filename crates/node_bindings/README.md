# fast-magic-string

Rust version of [magic-string](https://github.com/Rich-Harris/magic-string).
Inspired by [magic-string-rs](https://github.com/h-a-n-a/magic-string-rs).

## RoadMap

### Implemented

- [x] addSourcemapLocation
- [x] append
- [x] appendLeft
- [x] appendRight
- [x] clone
- [x] generateDecodedMap
- [x] generateMap
- [x] indent
- [x] insert
- [x] insertLeft
- [x] insertRight
- [x] move
- [x] overwrite
- [x] update
- [x] prepend
- [x] prependLeft
- [x] prependRight
- [x] remove
- [x] reset
- [x] slice
- [x] snip
- [x] toString
- [x] isEmpty
- [x] trimLines
- [x] trim
- [x] trimEnd
- [x] trimStart
- [x] hasChanged
- [x] replace
- [x] replaceAll

## Benchmarks

### Hardware Overview

    Model Name: MacBook Pro
    Model Identifier: MacBookPro18,3
    Chip: Apple M1 Pro
    Total Number of Cores: 8 (6 performance and 2 efficiency)
    Memory: 16 GB

### Version Overview

    node: 18.15.0
    magic-string: 0.30.17
    fast-magic-string: workspace

### Output

| Task Name         | ops/sec | Average Time (ns) | Margin  | Samples |
| ----------------- | ------- | ----------------- | ------- | ------- |
| magic-string      | 1,206   | 828588.7028143657 | ±15.48% | 123     |
| fast-magic-string | 1,698   | 588664.5233908365 | ±9.24%  | 172     |
