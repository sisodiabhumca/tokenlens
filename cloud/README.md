# TokenLens Cloud

Team-wide dashboard for TokenLens. Next.js 14 (App Router), deploys to Vercel.

```bash
pnpm install
pnpm --filter ./cloud dev
```

Production: set `DATABASE_URL` to a Postgres connection string (Neon recommended), then `vercel deploy`.
