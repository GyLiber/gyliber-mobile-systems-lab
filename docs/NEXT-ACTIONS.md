# Next actions — after v0.1.0

**State at stop (13 October 2026):** v0.1.0 code merged; GitHub CI passed; Render reported live; Neon free PostgreSQL connected. Source milestone is ready to be published; production authenticated flows and recoverability remain unverified.

## Priority 0 — Operations / acceptance (before storing actual research)
1. In the live browser, confirm the owner token authenticates; create a **synthetic** device, research entry, experiment, metadata artefact; change experiment status; reload lists; export JSON. Record date, result and failed steps without sharing tokens, logs or real device identifiers.
2. Check `/health` and anonymous `/api/v1/overview` (must be HTTP 401). Add an automated production smoke-check without exposing bearer credentials to untrusted workflows.
3. Retain a private, independent metadata export, document Neon free-tier resource limits and test restore into a separate throwaway database. Avoid sole-source storage; verify expiry and recovery expectations.
4. Review temporary bearer-token security: token rotation/revocation, HSTS/CSP and other security headers, rate limiting, access logs free of secrets, and narrow risk classification; resolve any critical finding before sensitive use.
5. Confirm GitHub release link, version tag target, deployed revision and that Render free plan remains selected.

## Next minor release — v0.2.0: evidence-backed research structure
**Acceptance goal:** from the browser, register a device component (hardware/software) and a source-linked observation; retrieve, filter and export both; record an architecture decision. Preserve unknowns instead of inventing hardware facts.

1. Define API schemas, migrations, indices, versioned export format, device/component foreign keys and explicit source-confidence evidence fields.
2. Implement component inventory and dated device observations (reference URL/text, provenance and uncertainty), with complete validation, tenant/owner checks and tests.
3. Implement dated architecture decision records (context, considered alternatives, rationale, consequences, superseding links).
4. Add bounded paginated listing/filtering and better research navigation; test against deliberately malformed inputs.
5. Add audit-safe create/update events, GitHub OAuth owner allowlist, HttpOnly sessions, CSRF and token retirement behind an ADR; do not introduce public self-registration.
6. Extend PostgreSQL E2E and production smoke, update threat model/OpenAPI, document migration and export compatibility; release only when all gates pass.

## Later releases
- **v0.3.0:** structured experimental procedures/runs, controlled variables, risk/rollback evidence and benchmark measurements.
- **v0.4.0:** recovery test records, known-good baselines, artifact integrity and tested backup/restore.
- **v0.5.0:** security findings, incident response, automated dependency/container scanning, operational metrics.
- **v0.6.0+**: evidence-driven local device/build integration; no web-initiated flashing or arbitrary shell execution.

## Scope discipline
Commit small reviewable units; require green CI, synthetic end-to-end demonstration, API/security documentation and release notes. Never claim verification without a record. The exact Samsung model/variant remains unknown until supported by actual device evidence.
