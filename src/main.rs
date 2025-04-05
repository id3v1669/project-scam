use clap::Parser;

mod iced_launch;
mod objects;
mod styles;
mod views;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Enable Debug Mode
    #[arg(short, long)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    unsafe {
        std::env::set_var("RUST_LOG", "warn");
        if args.debug {
            std::env::set_var("RUST_LOG", "debug");
        }
    }
    env_logger::init();
    log::debug!("Logger initialized");

    crate::iced_launch::custom_start();

    Ok(())
}
