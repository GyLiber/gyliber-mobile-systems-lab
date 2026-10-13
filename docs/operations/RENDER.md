# Render deployment preparation

`render.yaml` supplies a free Docker web service but does not create database credentials or select a Render account workspace.

Before deployment, the owner must select the intended Render workspace, provision/select one PostgreSQL database, and supply `DATABASE_URL` and an independently generated `LAB_OWNER_TOKEN` as secrets. Do not paste tokens into public discussion. Use HTTPS. The application runs migrations at startup; perform an independent JSON export before replacing a previous deployment.

**Free plan limitations:** Render free PostgreSQL has a 30-day expiry and does not provide managed backups. Free web services can sleep and have ephemeral local storage. Do not treat free hosting as durable evidence storage. Export JSON to an independent private location and rehearse restoration before depending on the service.

## Post-deploy smoke

Verify `GET /health` returns `status=ok`, unauthenticated `GET /api/v1/overview` returns HTTP 401, and the authenticated browser can create a synthetic device, research entry, experiment, artefact metadata, update experiment status, list records and export JSON. Do not tag the release before this check passes.
