# Security Policy

## Supported Versions

TokenLens is in public beta. Only the latest minor release receives security fixes.

| Version | Supported          |
| ------- | ------------------ |
| 0.2.x   | :white_check_mark: |
| < 0.2   | :x:                |

## Reporting a Vulnerability

Please **do not** open public GitHub issues for security reports.

Use one of the following private channels:

1. **GitHub Security Advisory** — preferred. Open a draft advisory at
   <https://github.com/sisodiabhumca/tokenlens/security/advisories/new>.
2. **Email** — `security@tokenlens.dev`. PGP key on request.

Please include:
- A description of the issue and its impact.
- Steps to reproduce, or a proof-of-concept payload.
- The TokenLens version (`tokenlens --version`) and host OS.
- Whether the issue affects the CLI, the cloud dashboard, the MCP server,
  the Vercel middleware, or the agent hooks.

We will acknowledge your report within **3 business days**, ship a fix or a
mitigation within **30 days** for high/critical issues, and credit you in the
release notes unless you ask us not to.

## Scope

In scope:
- The `tokenlens` CLI and its hooks (`hooks/`).
- The cloud dashboard (`cloud/`).
- The MCP server (`crates/tokenlens-mcp`).
- The Vercel AI SDK middleware (`packages/vercel`).
- The marketing site (`web/`) — only for actual security bugs, not content.

Out of scope:
- Vulnerabilities in third-party agents (Claude Code, Codex, Cursor, etc.).
  Please report those upstream.
- Denial of service caused by user-supplied filter rules.
- Issues that require physical access to the user's machine.
