use clap::Parser;
use notify_rust::Notification;

#[derive(Parser)]
#[command(name = "noti")]
#[command(about = "Send desktop notifications", long_about = None)]
struct Cli {
    /// Notification title
    #[arg(short, long)]
    title: Option<String>,

    /// Notification body
    #[arg(short, long)]
    body: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let title = cli.title.as_deref().unwrap_or("noti");
    let body = cli.body.as_deref().unwrap_or("");

    Notification::new()
        .summary(title)
        .body(body)
        .show()?;

    Ok(())
}
