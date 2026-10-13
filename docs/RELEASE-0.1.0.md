# v0.1.0 — initial vertical slice (release candidate)

## Implemented
Rust/Axum server, PostgreSQL migrations, owner-token-gated JSON API, dark responsive single-page UI, device profiles, research creation/listing, experiment proposals/listing/status updates, artefact metadata creation/listing, dashboard counts, JSON metadata export, container packaging and CI definition.

## Run locally
1. Install Rust, Cargo and PostgreSQL. Create an empty gmsl database.
2. Set DATABASE_URL and LAB_OWNER_TOKEN from .env.example (do not commit credentials). Use a cryptographically random token at least 32 characters.
3. Run `cargo test`, `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, then `cargo run`.
4. Open http://localhost:3000 and enter the owner token. Register a device, then use its UUID for linked records. Export the dataset with the UI.

## Safety and constraints
- Token auth is a temporary single-owner bootstrap, NOT the PDD's preferred GitHub OAuth. Share neither the token nor screenshots containing it.
- No public registration, device commands, flashing, firmware storage, or actual checksum verification.
- Experiment workflow supports proposed/in-progress/blocked; claiming a verified result is deliberately blocked until supporting evidence can be recorded. Structured runs and measurement records remain pending.
- Research, experiment and artefact listings appear in the UI; collections are currently bounded to the latest 100 entries without pagination.
- No data import or automated restore. Do not treat this as the only source of important research.
- The metadata export includes operational text and potentially sensitive entries; safeguard it.
- Render free PostgreSQL expires after 30 days and provides no managed backup; exporting is mandatory.
- No deployment, post-deployment smoke test or CI pass is claimed by this document.
- Release must not be tagged until CI builds and tests succeed and the owner confirms live, authenticated persistence and export.
- A simple bearer secret entered in a web page is not appropriate for high-sensitivity research; this build is limited to synthetic or non-sensitive research until OAuth/session security is implemented.

## Next development units
1. Fix all compiler, lint, security and database integration findings observed in CI.
2. Expand PostgreSQL integration tests for invalid inputs, foreign keys, experiment state transitions and export.
3. Implement GitHub OAuth owner allowlist, HttpOnly sessions, CSRF controls and audit events.
4. Implement record editing/listing and experiment status transitions.
5. Add versioned OpenAPI specification, rate limiting, CSP and deployment smoke automation.
6. Prove backup/restore, then consider the v0.2.0 research-structure release.

## Release gate
CI green; PostgreSQL migrations tested on fresh DB; unauthorized requests rejected; connected browser flow verified; real deploy tested; owner approval; then tag v0.1.0 and publish release notes.
