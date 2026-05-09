export default function Home() {
  return (
    <main>
      <div className="container">
        <header className="nav">
          <div className="brand">TokenLens</div>
          <nav>
            <a href="#features">Features</a>
            <a href="#install">Install</a>
            <a href="https://github.com/sisodiabhumca/tokenlens">GitHub</a>
          </nav>
        </header>

        <section className="hero">
          <h1>
            Watch every token your <span className="grad">AI coding agent</span> sends.
          </h1>
          <p className="lede">
            TokenLens is a token-aware observability and filtering layer for Claude Code, Codex CLI,
            Cursor, ChatGPT, Perplexity, and the Vercel AI SDK. It strips noise from agent prompts,
            tracks spend per model and project, and enforces budgets — locally, self-hosted, or in the cloud.
          </p>
          <div className="cta">
            <a className="btn primary" href="#install">Install</a>
            <a className="btn" href="https://github.com/sisodiabhumca/tokenlens">View source</a>
          </div>
          <pre className="install" id="install">
{`# one-liner installer (Linux + macOS)
curl -fsSL https://raw.githubusercontent.com/sisodiabhumca/tokenlens/main/scripts/install.sh | bash

# Homebrew (after the tap is published)
brew install sisodiabhumca/tokenlens/tokenlens

# wire up your agents
tokenlens init --agents claude,codex,cursor,perplexity`}
          </pre>
        </section>

        <section className="features" id="features">
          <div className="feature">
            <h3>Universal Hook Protocol</h3>
            <p>
              One JSON-line schema for every agent. Hooks for Claude Code, Codex, Cursor, Perplexity,
              Windsurf, and Cline ship in the box and delegate to <code>tokenlens hook recv</code>.
            </p>
          </div>
          <div className="feature">
            <h3>Filter 59 noisy commands</h3>
            <p>
              Forked from RTK with attribution. Strips noise from <code>cargo</code>, <code>npm</code>,
              <code>pytest</code>, <code>git status</code>, log tailers, and 50+ more — before tokens hit your model.
            </p>
          </div>
          <div className="feature">
            <h3>Per-model cost tracking</h3>
            <p>
              Built-in pricing for Claude Opus/Sonnet/Haiku, GPT-4o/4o-mini/5, Gemini 2.5 Pro, and free local
              models. SQLite by default, optional Postgres-backed cloud recorder.
            </p>
          </div>
          <div className="feature">
            <h3>Budgets with webhooks</h3>
            <p>
              Set daily/weekly/monthly caps per project. The cloud cron checks every six hours and posts
              to your webhook when a budget trips.
            </p>
          </div>
          <div className="feature">
            <h3>Vercel AI SDK middleware</h3>
            <p>
              <code>@tokenlens/vercel</code> drops into any AI SDK <code>generateText</code> /
              <code>streamText</code> call as a <code>LanguageModelV1Middleware</code> — zero infra.
            </p>
          </div>
          <div className="feature">
            <h3>MCP server</h3>
            <p>
              JSON-RPC over stdio exposing <code>compress</code>, <code>gain</code>, <code>lens.read</code>,
              and <code>lens.diff</code> so any MCP-aware client can query the lens directly.
            </p>
          </div>
        </section>

        <section className="agents">
          <h2>Works with the agents you already use</h2>
          <div className="agent-grid">
            <span className="tag">Claude Code</span>
            <span className="tag">Codex CLI</span>
            <span className="tag">Cursor</span>
            <span className="tag">ChatGPT</span>
            <span className="tag">Perplexity Computer</span>
            <span className="tag">Windsurf</span>
            <span className="tag">Cline</span>
            <span className="tag">Vercel AI SDK</span>
          </div>
        </section>

        <footer className="foot">
          <div>© {new Date().getFullYear()} TokenLens. MIT licensed.</div>
          <div>
            <a href="https://github.com/sisodiabhumca/tokenlens">GitHub</a>{" · "}
            <a href="https://github.com/sisodiabhumca/tokenlens/blob/main/SECURITY.md">Security</a>{" · "}
            <a href="https://github.com/sisodiabhumca/tokenlens/blob/main/CHANGELOG.md">Changelog</a>
          </div>
        </footer>
      </div>
    </main>
  );
}
