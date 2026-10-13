# Security policy

This application is a research prototype, not a device-flashing or security-hardening tool. Do not publish secrets, genuine device identifiers, private logs, or proprietary firmware.

## Reporting

Use GitHub's private vulnerability reporting feature if enabled on this repository, or contact the repository owner through a private channel. Do not post exploit details or sensitive traces in public issues.

## Scope and residual risks

The v0.1.0 candidate uses a temporary bearer token for single-owner access. GitHub OAuth with strict identity allowlisting, secure cookies, CSRF controls, audit events, rate limits, security headers, and external restore testing remain planned. Treat its hosted instances as low-sensitivity prototypes only.

Never deploy with the sample token. Generate a unique token of at least 32 random characters and provide it through host-managed secrets. Protect exported data separately.
