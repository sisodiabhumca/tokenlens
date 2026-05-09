# Self-hosting TokenLens Cloud

The dashboard and ingest API are a standard Next.js app and a Postgres database. You can run them anywhere.

## docker-compose (one command)

```bash
docker compose up -d
# Dashboard: http://localhost:3000
# Postgres:  localhost:5432  (user/pass: tokenlens / tokenlens)
```

The `cloud/db/schema.sql` file is mounted into the Postgres container's init dir, so the schema is created automatically the first time the container starts.

## Pointing the CLI at your instance

```bash
export TOKENLENS_CLOUD_URL=http://localhost:3000/api/ingest
export TOKENLENS_CLOUD_TOKEN=...   # optional, set INGEST_TOKEN on the server to enforce
tokenlens run -- git diff
```

## Kubernetes

A minimal manifest is in `deploy/k8s/`. Set `DATABASE_URL`, `INGEST_TOKEN`, and `CRON_SECRET` as `Secret`s and run `kubectl apply -k deploy/k8s/`.

## Vercel (managed)

```bash
cd cloud
vercel link
vercel env add DATABASE_URL
vercel env add INGEST_TOKEN
vercel env add CRON_SECRET
vercel deploy --prod
```

The `cloud/vercel.json` schedules `/api/budget/check` every 6 hours.

## Backups

Postgres `events` is the only durable data. Use `pg_dump` or your provider's automated backups.
