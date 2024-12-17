# nfl-rss-nlp :football:

![GitHub last commit](https://img.shields.io/github/last-commit/vaisriv/nfl-rss-nlp)
![GitHub Release](https://img.shields.io/github/v/release/vaisriv/nfl-rss-nlp)
![GitHub License](https://img.shields.io/github/license/vaisriv/nfl-rss-nlp)
![GitHub Repo stars](https://img.shields.io/github/stars/vaisriv/nfl-rss-nlp)

NFL Super Bowl prediction tool using NLP from RSS feeds

## Prerequesites

It is necessary to have the following:

- `rust`
- `libtorch`

These can be installed manually from [rustup](https://rustup.rs) and [pytorch](https://pytorch.org/get-started/locally/). As well, you must set the `libtorch` path.

These steps can be achieved on macOS (using Homebrew) through the `./requirements-macos.sh` script. The script assumes your user shell is POSIX compliant (as is `bash`, the default on macOS). I have included a `./requirements-macos.fish` script for users who use the `fish` shell.

If you require a manual method (i.e. in the case you are not on macOS or do not have Homebrew), the following should suffice (filling in your machine's information where needed):

```bash
<your-package-manager> install rust pytorch
export LIBTORCH=/your/path/to/pytorch
export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH
```

Likewise in fish:

```fish
<your-package-manager> install rust pytorch
set -x LIBTORCH /your/path/to/pytorch
set -x LD_LIBRARY_PATH $LIBTORCH/lib $LD_LIBRARY_PATH
```

## Installation and Running

Now we simply need to download the repository and we can run the program!

```bash
git clone "github.com/vaisriv/nfl-rss-nlp" ./nfl-rss-nlp
cd ./nfl-rss-nlp
cargo run --release
```

Of note, on first run, you will likely need to download the `rust-bert` Question-Answering model. This can be quite large, and make take a while. Be patient! Your Super Bowl answers are near!

## Acknowledgements

This project would not be possible without the following:

- [`rust-bert`](https://crates.io/crates/rust_bert)
- [`pytorch`](https://pytorch.org/)
- [`huggingface`](https://huggingface.co/)
