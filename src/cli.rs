use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "xstq")]
#[command(about = "Use xstq to query your code with XPath")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[arg(short, long)]
    pub verbose: bool,
    #[arg(short, long)]
    pub config: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Eval {
        #[arg(required = true)]
        path: Vec<String>,
        #[arg()]
        xpath: String,

        #[arg(long)]
        json: bool,
    },
    Analyze {
        #[arg(required = true)]
        path: Vec<String>,

        #[arg(long)]
        json: bool,
    },
}
