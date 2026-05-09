# @tokenlens/web

Marketing site for TokenLens. Next.js 14 (App Router), zero JS frameworks beyond React, deploys cleanly to Vercel.

```bash
pnpm --filter @tokenlens/web dev      # http://localhost:3094
pnpm --filter @tokenlens/web build
pnpm --filter @tokenlens/web start
```

## Deploy on Vercel

1. Import the repo into Vercel.
2. Set the project root to `web/`.
3. Build command: `pnpm --filter @tokenlens/web build`. Output: `.next`.
4. Live at <https://tokenlens-seven.vercel.app>.

No env vars required — the marketing site is fully static-friendly and links out to GitHub for the source.
