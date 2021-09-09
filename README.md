# I use arch btw

<div align="center">
    <h1>I use Arch btw</h1>
    <blockquote>
        <p>"I use Arch btw" but it's a Turing-complete programming language.</p>
    </blockquote>
    <a href="./iuab-cli/examples">Example programs</a>
    <span>|</span>
    <a href="https://github.com/overmighty/i-use-arch-btw/blob/master/docs/LANG_SPEC.md">Language specification</a>
    <span>|</span>
    <a href="https://github.com/overmighty/i-use-arch-btw/blob/master/docs/VM_SPEC.md">Virtual machine specification</a>
</div>


## Introduction
"I use Arch btw" is an esoteric programming language based on [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) in which the Brainfuck commands have been replaced with the following keywords:
`i`, `use`, `arch`, `linux`, `btw`, `by`, `the`, `way`.

This repository contains the source code for the Rust library and a CLI interpretter. Also there is an included wasm module.

## Installation
Note: By default, the executable is gonna be placed in `~/.cargo`.
```
$ cd iuab-cli
$ cargo install --path .
```

## Usage
To interpret a file containing `I use arch btw` source code:
```
$ iuab file.archbtw
```
To get help type:
```
$ iuab -h
```

## Thanks
Special thanks to @overmighty for this programming language.

## License
This project is licensed under the MIT license.