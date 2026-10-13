# Contributing

The project is early stage. Open a focused issue before introducing new device integrations or architectural dependencies. Use a short-lived branch, Conventional Commits and a pull request with a test plan.

Before requesting review: run `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets -- -D warnings`, `cargo test --locked`, and confirm the PostgreSQL integration smoke workflow is green. Never commit firmware binaries, personal logs, device identifiers, secrets, API keys, private vulnerability details or unlicensed material. Discuss incompatible API changes and threat-model impact explicitly.

AI-assisted code is subject to the same review, licensing, validation and security expectations as any contribution.
