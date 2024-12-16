mod context;
mod predict;
mod rss_read;

use clap::Parser;

#[derive(Parser, Debug)]
/// Welcome to the NFL Super Bowl Predictor. You can ask a question of your choosing, or simply use
/// the default.
struct Args {
    /// Question to ask NFL Predictor
    #[arg(short, long, default_value_t = ("What team is most likely to win the upcoming Super Bowl?").to_string())]
    question: String,

    /// Verbose Mode
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let context_str = context::get_context_str(args.verbose).await;
    let question_str = args.question;

    predict::predict(question_str, context_str.unwrap()).await;
}
