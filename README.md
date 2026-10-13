# GyLiber Mobile Systems Lab

A focused, evidence-driven laboratory for studying and improving a legacy Android Go device. **The phone's exact model and properties are not assumed.**

## Current status

Development version **0.1.0 release candidate**, not yet a validated hosted release.

The Rust/Axum + PostgreSQL application supports device profiles, research entries, experiment proposals, SHA-256 artefact metadata, authenticated overview and JSON export. It does **not** flash devices or run shell commands.

## Getting started

Requires Rust/Cargo and PostgreSQL. Create an empty `gmsl` database and provide:

- `DATABASE_URL`: PostgreSQL connection URL
- `LAB_OWNER_TOKEN`: randomly generated secret of at least 32 characters
- `PORT`: optional (default 3000)

Run `cargo test && cargo run`, then open http://localhost:3000. The token stays in the page's volatile memory, not localStorage. See [release candidate notes](docs/RELEASE-0.1.0.md) for limitations, tests and release gate.

The API is under `/api/v1`: `GET /devices`, `POST /devices`, `POST /research`, `POST /experiments`, `POST /artifacts`, `GET /overview`, `GET /export`. Protected calls require `Authorization: Bearer <LAB_OWNER_TOKEN>`. `GET /health` is public.

## Security and durability

Only use non-sensitive or synthetic records until the planned identity and hardening release. The current bootstrap token is not equivalent to OAuth. Never commit credentials, device identifiers, private logs or proprietary firmware. PostgreSQL holds metadata; export JSON regularly and protect exported files. Hosted deployment has not been verified.

## Architecture and deployment

Single Rust HTTP application, one PostgreSQL database, SQL migrations, one embedded static web page. Docker packaging and Render Blueprint are provided, requiring the owner to configure external secrets and database access. Review [release notes](docs/RELEASE-0.1.0.md) before deploying. This is a prototype, not a production security guarantee.

## Licensing and contributions

No open-source license has yet been granted. All rights reserved pending the product owner's explicit license selection. Changes should arrive through reviewed pull requests. Report security issues privately to the repository owner, not in public issues.

## AI assistance

AI tools contributed to design and implementation. GyLiber remains responsible for code review, validation, licensing, deployment decisions and security acceptance.
