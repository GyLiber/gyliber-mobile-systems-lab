# Production prototype — Render + Neon

**Web:** https://gyliber-mobile-systems-lab.onrender.com · Render workspace: My Workspace · Render service: `gyliber-mobile-systems-lab` · plan: **free**, region: **Oregon**.
**Database:** Neon Free PostgreSQL project `gyliber-mobile-systems-lab`, database `gmsl`, isolated from the existing Command Center database.

Set `DATABASE_URL` to Neon Postgres with SSL and `LAB_OWNER_TOKEN` to high-entropy random secret via Render's private environment settings. Never commit, log or share these values. Rotating the token invalidates the old bearer credential. Keep GitHub and hosting MFA enabled.

App startup connects to PostgreSQL then runs SQLx migrations, *before* binding port. Render marked initial deployment live on 2026-10-13, with app listening on port 10000. Automatic deployments from `main` are enabled. The `/health` endpoint is unauthenticated and exposes only status/version.

## Owner production smoke — outstanding
1. Open the app over HTTPS; enter owner token, click **Load overview**, confirm no error.
2. Register synthetic device `Test / Demo-001`; save the displayed UUID.
3. Create research, experiment and artefact metadata bound to that UUID. Use a **synthetic** SHA-256 hex value only for the demo.
4. Move the experiment from proposed to blocked; reload, confirm lists and overview match persisted records; download JSON export.
5. Request `/api/v1/overview` without credentials and confirm HTTP 401. Record date and outcomes in the release issue.
Do not publish the bearer token, real device identifiers or exported JSON in public GitHub issues.

## Data preservation
Neon free-tier data retention, quotas and suspension constraints should be checked on the owner's account. The web instance has ephemeral local storage; PostgreSQL is the data store. The export endpoint is not a complete backup and no re-import or tested database restore exists yet. Keep a separate private copy of exports and plan disaster recovery before real research.

## Troubleshooting and rollback
Check Render service events/deploy logs and Neon service status. Startup failures from bad connection string, SSL or migrations require inspecting protected logs without divulging passwords. Avoid destructive migrations. Re-deploy the previous known-good commit in Render after a bad code change, while preserving and independently backing up data. The free-plan service may sleep; cold starts are not guaranteed instantaneous.
