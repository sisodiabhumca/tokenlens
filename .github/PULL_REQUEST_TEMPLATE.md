<!-- Thanks for the contribution! Fill in the sections below; delete what isn't relevant. -->

## What

<!-- Short description of the change. -->

## Why

<!-- Link the issue, or explain the motivation. -->
Closes #

## How was this tested?

<!-- Check all that apply -->
- [ ] `cargo test --workspace`
- [ ] `pnpm --filter @tokenlens/vercel test`
- [ ] `pnpm --filter ./cloud build`
- [ ] `pnpm --filter ./web build`
- [ ] `bash tests/e2e/run.sh`
- [ ] Manual smoke test (describe below)

## Surfaces affected

- [ ] CLI (`tokenlens-core`)
- [ ] Hooks (`hooks/*`)
- [ ] Filters (`crates/tokenlens-core/filters/`)
- [ ] Cloud dashboard (`cloud/`)
- [ ] MCP server (`tokenlens-mcp`)
- [ ] Vercel middleware (`@tokenlens/vercel`)
- [ ] Marketing site (`web/`)
- [ ] Docs only

## Checklist

- [ ] CHANGELOG.md updated for user-visible changes
- [ ] Docs updated where needed (README, docs/)
- [ ] CI is green
