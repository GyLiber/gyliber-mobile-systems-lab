# GyLiber Mobile Systems Lab

Evidence-driven laboratory for studying legacy mobile systems, beginning with an Android Go device whose exact properties remain unknown until documented.

**Initial hosted release:** v0.1.0 · [Live prototype](https://gyliber-mobile-systems-lab.onrender.com) · [Release notes](docs/RELEASE-0.1.0.md) · [Next actions](docs/NEXT-ACTIONS.md).

The Rust/Axum application stores device profiles, research entries, proposed/blocked/in-progress experiments, artefact metadata, dashboard counts and JSON export in PostgreSQL. It neither flashes devices nor executes shell commands.

## Local development
Install Rust, Cargo and PostgreSQL. Create database `gmsl`, set `DATABASE_URL` and a randomly generated `LAB_OWNER_TOKEN` of at least 32 characters. Run `cargo test --locked && cargo run --locked`; open http://localhost:3000.

API routes under `/api/v1`: `GET/POST /devices`, `GET/POST /research`, `GET/POST /experiments`, `PATCH /experiments/{id}/status`, `GET/POST /artifacts`, `GET /overview`, `GET /export`. The authorization header is `Authorization: Bearer <owner-token>`; `GET /health` is public. Ordinary listings return up to 100 newest records.

## Deployment and limits
Render free Docker web service, with external Neon free PostgreSQL database `gmsl`. Render reported its initial deployment live on 13 October 2026. The deployed host is in Oregon; services can sleep on the free tier. The owner has not reported completing the authenticated browser smoke flow, so **production authenticated end-to-end behavior is not independently verified**. See [operations](docs/operations/RENDER.md).

Use only synthetic or non-sensitive data until proper OAuth identity, secure cookies, CSRF defense, rate limiting, production checks and backup/restore verification are delivered. The owner token is held only in browser memory, but bearer-token access is a temporary bootstrap and is not a complete security design. Never commit credentials, identifying device logs, private vulnerability findings or proprietary binaries. Protect exports independently.

## Project governance
No open-source licence is granted; all rights reserved pending a deliberate licence decision. Changes via reviewed pull requests and CI. Report security issues privately to the repository owner. AI tools assisted design and implementation; GyLiber contributors remain responsible for correctness, testing, security, licensing and deployment decisions.
