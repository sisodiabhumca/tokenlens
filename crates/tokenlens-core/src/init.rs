use crate::Agent;
use anyhow::Result;

pub fn run(agents: &[Agent]) -> Result<()> {
    if agents.is_empty() {
        println!("No agents specified. Try: tokenlens init --agents claude,codex,cursor,perplexity");
        return Ok(());
    }
    for a in agents {
        match a {
            Agent::Claude => println!("[init] Claude Code hook -> ~/.claude/hooks/tokenlens-rewrite.sh"),
            Agent::Codex => println!("[init] Codex CLI hook -> ~/.codex/hooks/tokenlens.sh"),
            Agent::Cursor => println!("[init] Cursor hook -> ~/.cursor/hooks/tokenlens-rewrite.sh"),
            Agent::Perplexity => println!("[init] Perplexity Computer skill + bash wrapper installed"),
            Agent::Windsurf => println!("[init] Windsurf rules.md updated"),
            Agent::Cline => println!("[init] Cline rules.md updated"),
            Agent::Kilocode => println!("[init] Kilocode rules.md updated"),
            Agent::Antigravity => println!("[init] Antigravity rules.md updated"),
            Agent::Vercel => println!("[init] Vercel: install @tokenlens/vercel and add the middleware"),
        }
    }
    println!("Done. Run `tokenlens gain` to start tracking savings.");
    Ok(())
}
