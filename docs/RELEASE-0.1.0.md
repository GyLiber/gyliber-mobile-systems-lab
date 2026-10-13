# v0.1.0 — Initial deployed vertical slice

**Release date:** 13 October 2026. **Status:** initial hosted prototype, known limitations. **Production:** https://gyliber-mobile-systems-lab.onrender.com

## Delivered
- Rust/Axum API and SQLx PostgreSQL schema migrations, deployed as non-root Docker image.
- Dark responsive owner-facing UI; create/list device profile, research, experiments and artefact metadata.
- Experiment statuses `proposed`, `in_progress`, `blocked`; no fabricated verification of procedures.
- Database-backed overview counters; metadata JSON export; SHA-256 metadata field validation.
- Single-owner bearer-token gate on protected API routes (temporary prototype mechanism).
- CI with Rust formatting, strict Clippy, unit tests, release build and PostgreSQL-backed full create/list/status/export smoke flow, including unauthorized rejection.
- `Cargo.lock` for locked dependency resolution, security/runbook documentation and free-host deployment configuration.

## Verification record
- PR #1 merged into main as `090871e19341fb43584a9593972690db78de92f9`; last PR head `4b1749049f10da19e9561f8287d2fbc0c65ef336` passed two CI workflow runs, including PostgreSQL-backed E2E.
- Render Docker deployment of merge commit reported **live** on 2026-10-13; logs include `GMSL listening on 10000` and Render's live announcement.
- Neon free database `gmsl` created by owner. Successful server startup implies its configured PostgreSQL connection and startup migrations completed.
- **Not verified:** production authenticated user flow (owner has not confirmed after browser testing); independent production HTTP smoke was not available from the previous validation environment. Do not represent these as passed.
- Release publication is a **source/hosting milestone**, not production security certification or acceptance of sensitive real-device research.

## Explicit exclusions and risk
Bearer-token authentication is a limited interim design, not GitHub OAuth; no per-user audit trail, CSRF/session protection, rate limiting, full input threat audit or external restore procedure. Data export is not automated backup. Free infrastructure can sleep, impose quotas or change. Do not store private or safety-critical research as the only copy.

No device flashing, remote command execution, modem modifications, firmware binary uploads, genuine SHA-256 content verification, verified recovery, or benchmarking. Listings have 100-record cap and no pagination. The application cannot represent an experiment as `verified` until future evidence-linked verification workflows are implemented.

## Reproduce
With Rust and PostgreSQL: `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets -- -D warnings`, `cargo test --locked`, `cargo build --locked --release`. Set `DATABASE_URL` and `LAB_OWNER_TOKEN` via private environment configuration; run `cargo run --locked`. See README and [operations](operations/RENDER.md).

## Next actions
See [NEXT-ACTIONS.md](NEXT-ACTIONS.md) for prioritized release follow-up and v0.2.0 scope.
