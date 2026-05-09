//! TokenLens core CLI.
//!
//! Subcommands:
//!   tokenlens init        Install hooks for one or more agents
//!   tokenlens rewrite     Rewrite a shell command for token efficiency
//!   tokenlens gain        Show token-savings analytics
//!   tokenlens budget      View / set monthly spend cap
//!   tokenlens mcp serve   Run the MCP server
//!   tokenlens hook recv   Read UHP JSON lines on stdin and respond
//!   tokenlens import-rtk  Import an existing RTK SQLite tracking DB

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};

mod analytics;
mod budget;
mod hook;
mod init;
mod rewrite;
mod tracking;

#[derive(Parser)]
#[command(
    name = "tokenlens",
    version,
    about = "Universal context-window optimizer and observability layer for AI coding agents"
)]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Install hooks for one or more agents.
    Init {
        /// Comma-separated list of agents.
        #[arg(long, value_delimiter = ',')]
        agents: Vec<Agent>,
    },
    /// Rewrite a shell command for token efficiency.
    Rewrite {
        /// The command to rewrite.
        #[arg(trailing_var_arg = true)]
        cmd: Vec<String>,
    },
    /// Show token-savings analytics.
    Gain {
        /// Group by one or more dimensions: model, repo, agent.
        #[arg(long, value_delimiter = ',')]
        by: Vec<String>,
        /// Output format.
        #[arg(long, default_value = "table")]
        format: String,
    },
    /// View or set monthly spend cap.
    Budget {
        /// Set monthly budget in USD.
        #[arg(long)]
        set_monthly: Option<f64>,
    },
    /// MCP server commands.
    Mcp {
        #[command(subcommand)]
        sub: McpCmd,
    },
    /// Universal Hook Protocol server (reads JSON lines on stdin).
    Hook {
        #[command(subcommand)]
        sub: HookCmd,
    },
    /// Import an existing RTK tracking DB.
    ImportRtk {
        /// Path to RTK tracking.db (default: ~/.local/share/rtk/tracking.db).
        #[arg(long)]
        from: Option<String>,
    },
}

#[derive(Subcommand)]
enum McpCmd {
    /// Start the MCP server (stdio transport).
    Serve,
}

#[derive(Subcommand)]
enum HookCmd {
    /// Receive UHP requests on stdin, write responses on stdout.
    Recv,
}

#[derive(Debug, Clone, Copy, PartialEq, ValueEnum)]
pub enum Agent {
    Claude,
    Codex,
    Cursor,
    Perplexity,
    Windsurf,
    Cline,
    Kilocode,
    Antigravity,
    Vercel,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Cmd::Init { agents } => init::run(&agents),
        Cmd::Rewrite { cmd } => rewrite::run(cmd.join(" ")),
        Cmd::Gain { by, format } => analytics::gain(&by, &format),
        Cmd::Budget { set_monthly } => budget::run(set_monthly),
        Cmd::Mcp { sub } => match sub {
            McpCmd::Serve => {
                eprintln!("[tokenlens] MCP server: delegating to tokenlens-mcp");
                Ok(())
            }
        },
        Cmd::Hook { sub } => match sub {
            HookCmd::Recv => hook::recv(),
        },
        Cmd::ImportRtk { from } => tracking::import_rtk(from.as_deref()),
    }
}
