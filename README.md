# gignore

gignore is a command line tool to help you easily create `.gitignore` files for your projects, so you can stop uploading dependencies and irrelevant files to your remotes.

The gitignore templates are extracted in real time from https://github.com/github/gitignore, a collection of useful gitignore templates maintained by GitHub.

## Installation

To install this tool, you will need to have [Cargo, the Rust package manager](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed. Once you have it, you can simply run `cargo install gignore`.

## Getting started

To get started, you can run `gignore ls` to check all the available language and technologies for gitignore templates.

You can then create a template for you by running `gignore new <language>`, with your favorite language or technology.

## LICENSE

[MIT](./LICENSE)
