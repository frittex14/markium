# Markium

Markium is a CLI tool for previewing markdown files,
that tries it's best to mimic GitHub's styling.

## WARNING

Markium is still in early development! Things like live preview server are not
implemented yet, styling is not a 100% match (especially code highlighting),
and it needs a lot of field testing before it can safely be called stable.

However, it's usable. Very usable. I tested it with many popular README's,
and most of the times it was a >98% match (excluding code highlighting).
I feel safe to say that basic markdown parsing is already 100% complete,
and only things like images, custom scripts, and other very specific GitHub
features are missing. That's why I decided to already release it.

## Examples

See [the examples directory](https://github.com/frittex14/markium/tree/master/examples)

## Usage

For details on how to use the CLI, and all available options, run:

```bash
markium --help
```

## Installation

As the tool is still in early development, the only way of installing it,
is to build it from source and manually link the binary to `$PATH`.

```bash
git clone https://github.com/frittex14/markium.git # clone the repo
cd markium && cargo build --release # build the binary
ln -sf $PWD/target/release/markium $HOME/.local/bin/markium # link the binary to $PATH
```

This way, it can be updated with:

```bash
git pull && cargo build --release # pull newest version and build the binary
```

## TODOs

- [ ] Add live preview server
- [ ] Add option to auto-open output in browser

## Contributing

See [How to Contribute to Open Source](https://opensource.guide/how-to-contribute/) and [CONTRIBUTING.md](https://github.com/frittex14/markium/blob/master/CONTRIBUTING.md)

## License

MIT License (see [LICENSE](https://github.com/frittex14/markium/raw/master/LICENSE) or [https://opensource.org/license/MIT](https://opensource.org/license/MIT))
