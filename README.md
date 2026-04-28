# Zed Go Template Extension

A smart and fast language extension for the [Zed Editor](https://zed.dev/) that
provides first-class support for Go Templates (`text/template` and `html/template`).

## Features

- **Syntax Highlighting:** Precise highlighting for Go template directives (`{{ }}`) powered by a modern tree-sitter parser.
- **Syntax Check:** Checking if synatx is valid through gopls

## Installation

Via Zed [extension store](https://zed.dev/extensions/go-template).

## Requirements

For autocompletion and error detection to work, the official Go Language Server must be installed and available in your `$PATH`:

```sh
go install golang.org/x/tools/gopls@latest
```
