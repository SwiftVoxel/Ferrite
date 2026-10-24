# 🤝 Contributing to Ferrite

Thanks for your interest in contributing to Ferrite! This document covers how to get involved and what's expected of contributions.

## 🛠️ Getting Started

1. Fork the repository and clone your fork locally.
2. Make sure you have a recent stable Rust toolchain installed (via [rustup](https://rustup.rs/)).
3. Build the project with `cargo build` and run it with `cargo run`.
4. Create a branch for your change and commit your work there.
5. Open a pull request against the `main` branch, with a clear description of what the change does and why.

## 📜 Code Guidelines

- Keep code formatted with `cargo fmt` and check it with `cargo clippy` before submitting.
- Keep pull requests focused — unrelated changes should be split into separate PRs.
- Write clear commit messages describing what changed and why.
- If you're adding a new feature or changing existing behavior, briefly explain your reasoning in the pull request description.

## 🤖 AI-Generated Code

The use of AI-assisted or AI-generated code in contributions is permitted. However, **contributors are responsible for the code they submit**, regardless of how it was produced. This means:

- You must understand what the code does before submitting it. Don't submit code you can't explain.
- You are responsible for testing it and verifying it works as intended.
- You are responsible for ensuring it doesn't introduce bugs, security issues, or license conflicts (including making sure any AI-generated code doesn't reproduce copyrighted or incompatibly-licensed material).
- Reviewers may ask you to explain or justify any part of a submission, AI-assisted or not. Treat AI tools as a tool that helps you write code, not as a substitute for your own review and judgment.

Submissions that appear to be low-effort, unreviewed AI output — regardless of whether AI was involved — may be asked for revision or declined.

## ⛔ Reporting Issues

If you find a bug or want to request a feature, please open an issue describing:

- What you expected to happen
- What actually happened (for bugs)
- Steps to reproduce, if applicable
- Your environment (OS, Rust version) if relevant

## 📄 License

By contributing to Ferrite, you agree that your contributions will be licensed under the project's [GNU General Public License v3.0](LICENSE).
