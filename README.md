# Code Cleaner Wrasse

Code Cleaner Wrasse (CCW) is a CLI tool that uses a local LLM (via Ollama) to analyze source code, review diffs, detect issues, and generate structured summaries for commits and task management workflows.

It provides LLM-assisted analysis of source code, including checks for security issues, correctness problems, performance pitfalls, documentation gaps, and acceptance-criteria compliance - all without sending code or diffs to external services.

## Why Code Cleaner Wrasse?

Modern development moves fast, and large codebases make it difficult to maintain consistent code quality, performance, and documentation. Code Cleaner Wrasse helps developers keep their projects healthy by providing automated, LLM-powered insights - all while keeping your code local, private, and under your control.

Code Cleaner Wrasse is designed to:
* Improve code quality by detecting logic issues, security risks, and unsafe patterns.
* Highlight performance bottlenecks such as unnecessary allocations, cloning, or inefficient loops.
* Streamline code reviews by generating clear summaries of diffs, commit messages, and task descriptions.
* Support planning and QA with automatically generated acceptance criteria and testing guidance.
* Reduce cognitive load so developers can focus on building features instead of manually checking boilerplate details.
* Work entirely offline, relying on your local LLM through Ollama - no cloud, no external API calls.

Whether you’re working solo, in a team, or on open source, Code Cleaner Wrasse gives you fast, consistent feedback and helps keep your codebase clean, safe, and easy to maintain.

## Installation

To install the program, run the command below in the directory that contains the repository:

```sh
cargo install --force --locked --path .
```

## Requirements

* [Rust toolchain](https://rust-lang.org/tools/install/).
* [Ollama](https://ollama.com/download).
* OLLAMA_HOST environment variable must point to your running Ollama server.
* Default model - pull the recommended default model:

```sh
ollama pull qwen2.5-coder:14b
```

You may use a different model with the `--model` parameter.

## Disclaimer

Code Cleaner Wrasse analyzes code and generates summaries, reviews, and suggestions using a local language model. It does not guarantee correctness, completeness, or security of its output.

All results should be reviewed by a human before being used in production systems. The authors of this project are not liable for any damages, bugs, data loss, or security issues resulting from the use of this tool.

Do not use this tool on code or data that you are not authorized to analyze.

## Usage

CCW provides several modes, each focused on a specific code review or task management workflow.

### Common Parameters

* `--max-attempts` - maximum number of retry attempts when communicating with the Ollama server.

* `--model` - overrides the default model (qwen2.5-coder:14b).

* `--skip-larger` - skips files requiring a context window larger than the hardware can support. Example: 30000 works for systems with 8 GB VRAM + 64 GB RAM when offloading qwen2.5-coder:14b.

## Modes

### Checker

Analyzes code for correctness and security issues:
* cryptographic errors
* documentation errors
* logic errors
* overflow risks
* security bugs
* unsafe Rust usage

Example:

```sh
ccw --mode=checker --skip-larger=30000 -d ./src/
```

Options:

* `--dir` - analyze a directory recursively
* `--file` - analyze a single file
* `--start-line`, `--end-line` - analyze a selected section of a file

Useful when working around context window limits on large files.

### Commit review

Generates a review of the code changes in a diff:

```sh
git diff main | ccw --mode=commit_review
```

### Commit summary

Generates a concise commit-message-ready summary:

```sh
git diff main | ccw --mode=commit_summary
```

### Performance

Detects performance-related issues:
* unnecessary allocations
* allocations inside loops
* excessive cloning
* inefficient algorithms
* unbuffered I/O
* heavy operations inside hot loops

Example:

```sh
ccw --mode=performance --skip-larger=30000 -d ./src/
```

### Task comment

Generates a task-style comment summarizing changes and describing how to test them:

```sh
git diff main | ccw --mode=task_comment
```

### Task criteria check

Checks whether code changes meet predefined acceptance criteria.

Provide criteria via a file:

```sh
git diff main | ccw --mode=task_criteria_check --file=../tmp/acceptance_criteria.txt
```

### Task description

Generates a task title, a structured task description, and automatically generated acceptance criteria based on the code changes.

This output is suitable for issue trackers and task-planning systems.

```sh
git diff main | ccw --mode=task_description
```

## Contributing

Thank you for your interest in improving Code Cleaner Wrasse!

At this time, I am not accepting code contributions or pull requests. This project is maintained as a personal tool that I share publicly in the hope that others may find it useful.

## License

This project is licensed under the MIT License.

See the [LICENSE](LICENSE) file in the repository for full license text.
