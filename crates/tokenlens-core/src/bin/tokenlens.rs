//! `tokenlens` binary — thin CLI over the tokenlens-core library.

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use tokenlens_core::filter::CompressionLevel;
use tokenlens_core::recorder::{Event, Recorder, SqliteRecorder};
use tokenlens_core::{budget, cmds, hook, recorder, rewrite, semantic, tracking};

#[derive(Parser)]
#[command(name = "tokenlens", version, about = "Universal context-window optimizer for AI coding agents")]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Install hooks for one or more agents.
    Init {
        #[arg(long, value_delimiter = ',')]
        agents: Vec<Agent>,
    },
    /// Rewrite a shell command (RTK exit-code protocol).
    Rewrite {
        #[arg(trailing_var_arg = true)]
        cmd: Vec<String>,
    },
    /// Run any tool, compress its output before printing.
    #[command(name = "run", alias = "wrap")]
    Run {
        /// Compression level.
        #[arg(long, default_value = "minimal")]
        level: String,
        #[arg(trailing_var_arg = true, required = true)]
        cmd: Vec<String>,
    },
    /// Read a file with token-aware filtering.
    Read {
        path: PathBuf,
        #[arg(long, default_value = "minimal")]
        level: String,
        #[arg(long)]
        max_lines: Option<usize>,
    },
    /// Fetch a URL and print the compressed body.
    Fetch {
        url: String,
        #[arg(long, default_value = "minimal")]
        level: String,
    },
    /// Compress arbitrary text from stdin (or `--text`).
    Compress {
        #[arg(long, default_value = "minimal")]
        level: String,
        #[arg(long)]
        text: Option<String>,
        #[arg(long)]
        cmd: Option<String>,
        #[arg(long)]
        semantic: bool,
        #[arg(long)]
        target: Option<u64>,
    },
    /// Show token-savings analytics.
    Gain {
        #[arg(long, value_delimiter = ',')]
        by: Vec<String>,
        #[arg(long, default_value = "table")]
        format: String,
    },
    /// View / set / check the monthly budget.
    Budget {
        #[arg(long)]
        set_monthly: Option<f64>,
        #[arg(long)]
        webhook: Option<String>,
        #[arg(long)]
        check: bool,
    },
    /// MCP server commands.
    Mcp {
        #[command(subcommand)]
        sub: McpCmd,
    },
    /// Universal Hook Protocol server.
    Hook {
        #[command(subcommand)]
        sub: HookCmd,
    },
    /// Import an existing RTK tracking DB.
    ImportRtk {
        #[arg(long)]
        from: Option<String>,
    },
}

#[derive(Subcommand)]
enum McpCmd { Serve }

#[derive(Subcommand)]
enum HookCmd { Recv }

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Agent {
    Claude, Codex, Cursor, Perplexity, Windsurf, Cline, Kilocode, Antigravity, Vercel,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Cmd::Init { agents } => init(&agents),
        Cmd::Rewrite { cmd } => rewrite::run(cmd.join(" ")),
        Cmd::Run { level, cmd } => run_proxy(&level, &cmd),
        Cmd::Read { path, level, max_lines } => {
            let level = level.parse::<CompressionLevel>().map_err(anyhow::Error::msg)?;
            let outcome = cmds::read::read_file(&path, level, max_lines)?;
            print_outcome(&outcome);
            Ok(())
        }
        Cmd::Fetch { url, level } => {
            let level = level.parse::<CompressionLevel>().map_err(anyhow::Error::msg)?;
            let outcome = cmds::fetch::fetch_url(&url, level)?;
            print_outcome(&outcome);
            Ok(())
        }
        Cmd::Compress { level, text, cmd, semantic: do_semantic, target } => {
            let level = level.parse::<CompressionLevel>().map_err(anyhow::Error::msg)?;
            let input = match text {
                Some(t) => t,
                None => {
                    use std::io::Read;
                    let mut buf = String::new();
                    std::io::stdin().read_to_string(&mut buf)?;
                    buf
                }
            };
            let mut outcome = tokenlens_core::compress_text(&input, cmd.as_deref(), level);
            if do_semantic {
                if let Some(backend) = semantic::default_backend() {
                    let target = target.unwrap_or(outcome.compressed_tokens / 2);
                    if let Ok(s) = backend.summarize(&outcome.output, target) {
                        outcome = tokenlens_core::filter::FilterOutcome::from(&input, s, format!("{}+semantic", outcome.strategy));
                    }
                }
            }
            print_outcome(&outcome);
            Ok(())
        }
        Cmd::Gain { by, format } => gain(&by, &format),
        Cmd::Budget { set_monthly, webhook, check } => {
            if let Some(usd) = set_monthly { return budget::set_monthly(usd); }
            if let Some(url) = webhook { return budget::set_webhook(url); }
            if check { return budget::check(); }
            budget::show()
        }
        Cmd::Mcp { sub: McpCmd::Serve } => {
            // Re-exec the standalone tokenlens-mcp binary if present in PATH;
            // otherwise call the in-process handler from tokenlens-mcp lib (future).
            eprintln!("Run `tokenlens-mcp` directly for the MCP stdio server.");
            Ok(())
        }
        Cmd::Hook { sub: HookCmd::Recv } => hook::recv(),
        Cmd::ImportRtk { from } => tracking::import_rtk(from.as_deref()),
    }
}

fn run_proxy(level: &str, cmd: &[String]) -> Result<()> {
    let level = level.parse::<CompressionLevel>().map_err(anyhow::Error::msg)?;
    if cmd.is_empty() { anyhow::bail!("missing command"); }
    let program = &cmd[0];
    let args = cmd[1..].to_vec();
    let result = cmds::run_proxied(program, &args, level)?;
    println!("{}", result.outcome.output);
    // Best-effort recorder write — never fail the caller.
    let _ = record_event(&format!("{} {}", program, args.join(" ")), &result.outcome);
    std::process::exit(result.status);
}

fn record_event(cmd: &str, outcome: &tokenlens_core::filter::FilterOutcome) -> Result<()> {
    let tracker = tracking::Tracker::open_default()?;
    let rec = SqliteRecorder { tracker };
    rec.record(&Event {
        ts: chrono::Utc::now().timestamp(),
        cmd: cmd.to_string(),
        input_tokens: outcome.original_tokens,
        output_tokens: outcome.compressed_tokens,
        saved_tokens: outcome.saved_tokens,
        dollars_saved: dollars_for(outcome.saved_tokens, std::env::var("TOKENLENS_MODEL").ok().as_deref()),
        agent: std::env::var("TOKENLENS_AGENT").ok(),
        model: std::env::var("TOKENLENS_MODEL").ok(),
        repo: std::env::current_dir().ok().map(|p| p.display().to_string()),
    })?;
    if std::env::var("TOKENLENS_CLOUD_URL").is_ok() {
        let cloud = recorder::CloudRecorder::new(
            std::env::var("TOKENLENS_CLOUD_URL").unwrap(),
            std::env::var("TOKENLENS_CLOUD_TOKEN").ok(),
        );
        let _ = cloud.record(&Event {
            ts: chrono::Utc::now().timestamp(),
            cmd: cmd.to_string(),
            input_tokens: outcome.original_tokens,
            output_tokens: outcome.compressed_tokens,
            saved_tokens: outcome.saved_tokens,
            dollars_saved: dollars_for(outcome.saved_tokens, std::env::var("TOKENLENS_MODEL").ok().as_deref()),
            agent: std::env::var("TOKENLENS_AGENT").ok(),
            model: std::env::var("TOKENLENS_MODEL").ok(),
            repo: std::env::current_dir().ok().map(|p| p.display().to_string()),
        });
    }
    Ok(())
}

/// Approximate per-1M-token input price by model; returns dollars *saved*.
fn dollars_for(saved_tokens: u64, model: Option<&str>) -> f64 {
    let per_million = match model.unwrap_or("") {
        m if m.contains("opus") => 15.0,
        m if m.contains("sonnet") => 3.0,
        m if m.contains("haiku") => 0.80,
        m if m.contains("gpt-4o-mini") => 0.15,
        m if m.contains("gpt-4o") => 5.0,
        m if m.contains("gpt-5") => 8.0,
        m if m.contains("gemini-2.5-pro") => 3.5,
        m if m.contains("gemini") => 1.25,
        m if m.contains("llama") => 0.0,
        _ => 2.0,
    };
    (saved_tokens as f64) * per_million / 1_000_000.0
}

fn print_outcome(o: &tokenlens_core::filter::FilterOutcome) {
    println!("{}", o.output);
    eprintln!(
        "[tokenlens] strategy={} saved {} tokens ({:.1}%)",
        o.strategy, o.saved_tokens, o.savings_pct
    );
}

fn gain(by: &[String], format: &str) -> Result<()> {
    let tracker = tracking::Tracker::open_default()?;
    let summary = tracker.summary()?;

    if format == "json" {
        let mut payload = serde_json::to_value(&summary)?;
        if !by.is_empty() {
            let mut pivots = serde_json::Map::new();
            for dim in by { pivots.insert(dim.clone(), serde_json::to_value(tracker.pivot(dim).unwrap_or_default())?); }
            payload["pivots"] = serde_json::Value::Object(pivots);
        }
        println!("{}", serde_json::to_string_pretty(&payload)?);
        return Ok(());
    }

    use colored::Colorize;
    println!("{}", "TokenLens — savings".bold());
    println!("Total commands : {}", summary.commands);
    println!("Input tokens   : {}", fmt(summary.input_tokens));
    println!("Output tokens  : {}", fmt(summary.output_tokens));
    println!("Tokens saved   : {} ({:.1}%)", fmt(summary.saved_tokens), summary.savings_pct());
    println!("Est. $ saved   : ${:.2}", summary.dollars_saved);

    for dim in by {
        match tracker.pivot(dim) {
            Ok(rows) => {
                println!("\n{} breakdown:", dim);
                for (k, v) in rows { println!("  {:<32} {}", k, fmt(v)); }
            }
            Err(e) => eprintln!("[tokenlens] {e}"),
        }
    }
    Ok(())
}

fn fmt(n: u64) -> String {
    let n = n as f64;
    if n >= 1.0e9 { format!("{:.1}B", n / 1.0e9) }
    else if n >= 1.0e6 { format!("{:.1}M", n / 1.0e6) }
    else if n >= 1.0e3 { format!("{:.1}K", n / 1.0e3) }
    else { format!("{n:.0}") }
}

fn init(agents: &[Agent]) -> Result<()> {
    if agents.is_empty() {
        println!("Try: tokenlens init --agents claude,codex,cursor,perplexity");
        return Ok(());
    }
    for a in agents {
        let line = match a {
            Agent::Claude => "Claude Code hook -> ~/.claude/hooks/tokenlens-rewrite.sh",
            Agent::Codex => "Codex CLI hook -> ~/.codex/hooks/tokenlens.sh",
            Agent::Cursor => "Cursor hook -> ~/.cursor/hooks/tokenlens-rewrite.sh",
            Agent::Perplexity => "Perplexity Computer skill + bash wrapper",
            Agent::Windsurf => "Windsurf rules.md updated",
            Agent::Cline => "Cline rules.md updated",
            Agent::Kilocode => "Kilocode rules.md updated",
            Agent::Antigravity => "Antigravity rules.md updated",
            Agent::Vercel => "Vercel: install @tokenlens/vercel and add the middleware",
        };
        println!("[init] {line}");
    }
    println!("Done. Run `tokenlens gain` to start tracking savings.");
    Ok(())
}
