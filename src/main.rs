mod github;
mod interactive;
mod issues_count;
mod errors;

use std::error::Error;

use clap::Parser;
use github::GhClient;

#[derive(Parser, Debug)]
struct Args {
    /// using interactive mode or not (default: true)
    /// if true all other flag is ignored
    #[arg(short, long)]
    #[clap(default_value = "true")]
    interactive: bool,

    /// authenticated github username
    #[arg(short, long)]
    username: Option<String>,

    /// gitgub api token, create one by going to settings > develper settings > personal access token on github
    #[arg(short, long)]
    token: Option<String>,

    /// github organization name
    #[arg(short, long)]
    org: Option<String>,

    /// github project number
    #[arg(short, long)]
    project_num: Option<i64>,

    // start date for filtering issues
    #[arg(short, long)]
    start_date: Option<String>,

    // end date for filtering issues
    #[arg(short, long)]
    end_date: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut args = Args::parse();

    let mut gh_client: GhClient = GhClient::new(
        args.username.clone().unwrap_or_default(),
        args.token.clone().unwrap_or_default(),
    );

    if args.interactive {
        interactive::run_interactive(&mut args, &mut gh_client).await?;
    }

    let count = issues_count::issues_count(&args, &gh_client).await?;

    println!("{:?}", count);

    Ok(())
}

