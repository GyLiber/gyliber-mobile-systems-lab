# ADR-0001 — Single-owner bootstrap for the initial vertical slice

**Status:** Accepted for low-sensitivity prototype only; must be replaced before production-grade usage.

**Context:** The first version needs a reproducible, owner-gated research hub. The preferred GitHub OAuth identity integration requires an external OAuth app, callback URL, configured allowlist, and secrets not yet supplied. A public write API without access control is unacceptable.

**Decision:** For v0.1.0 candidate, require a host-supplied high-entropy `LAB_OWNER_TOKEN` on protected API requests. Reject missing/invalid authorization. Keep the browser token only in page memory, not persistent browser storage. Record the OAuth implementation as a mandatory hardening item.

**Tradeoffs:** Bearer token use lacks account-level audit identities and independent revocation, and a stolen token grants owner access until rotated. Access to the public home page is not restricted, but protected resources are. Only synthetic or non-sensitive records should be stored until stronger controls are verified. Use HTTPS and secrets management for hosted use.

**Consequences:** No anonymous writes. No public registration. OAuth allowlist, HttpOnly sessions, CSRF and security-header review are prerequisites for any security-sensitive operations or user expansion.
