# Code Cleaner Wrasse

## Installation

To install the program, run the command below in the directory that contains the repository.

```sh
cargo install --force --locked --path .
```

You need to have installed [Rust toolchain](https://rust-lang.org/tools/install/).

You need to have installed [Ollama](https://ollama.com/download).

Make sure that you have defined OLLAMA_HOST environment variable.

You also need to pull a default model that is used.

```sh
ollama pull qwen2.5-coder:14b
```

## Usage

There are a few modes that are supported.

### Common parameters

--max-attempts - this parameter defines the maximum number of request attempts that are made when trying to communicate with the Ollama server.

--model - this parameter allows to select another model than the default qwen2.5-coder:14b.

--skip-larger - this parameter allows skipping files that require a context window that is larger than the context window that could be handled by the hardware. 30000 is a value that works for 8 GB of VRAM and 64 GB of RAM with offloading of qwen2.5-coder:14b.

### Checker

In this mode, the code is checked for the following problems:
- cryptographic errors
- documentation errors
- logic errors
- overflow errors
- security bugs
- unsafe code bugs

```sh
ccw --mode=checker --skip-larger=30000 -d /home/michal/projects/forks/rust/compiler/rustc_borrowck/
```

--dir - allows checking the whole directory with subdirectories.

--file - allows checking a single file.

--start-line, --end-line - allows to select certain lines from the file to check. These settings allow us to workaround the problem with context window size on hardware that is not able to support larger context windows that are required for larger files.

### Performance

In this mode, the code is checked for the problems that may cause worse performance of the program.

```sh
ccw --mode=performance --skip-larger=30000 -d /home/michal/projects/forks/rust/compiler/rustc_borrowck/
```

### Commit summary

In this mode, the tool generates a summary of the code changes that can be used as a template for a commit message.

```sh
git diff main | ccw --mode=commit_summary
```
