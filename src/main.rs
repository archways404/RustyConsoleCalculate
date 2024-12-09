use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Tz;
use clap::{Parser, Subcommand};
use meval::eval_str;

#[derive(Parser)]
#[command(
    name = "RCC",
    version = "0.0.2",
    about = "A CLI tool for performing calculations and managing timestamps",
    long_about = "RCC is a versatile CLI tool that allows you to:\n\
                  1. Perform basic mathematical calculations by providing an expression as a string, e.g., '1 + 2'.\n\
                  2. Convert timestamps to a specific timezone using the 'ctz' command.\n\
                  3. Translate timestamps into human-readable formats using the 'ctr' command.\n\
                  4. Get the current Unix timestamp using the 'ct' command."
)]
struct Cli {
    /// Subcommand for timestamp-related operations
    #[command(subcommand)]
    command: Option<Commands>,

    /// Mathematical expression to evaluate (default behavior)
    #[arg(
        required = false,
        help = "Mathematical expression to evaluate, e.g., '13 + 14 * 2'"
    )]
    expression: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert a Unix timestamp to a specific timezone
    #[command(aliases = ["ctz"], long_about = "Convert a Unix timestamp (seconds since epoch) to a given timezone, e.g., America/New_York.")]
    Ctz {
        /// The timestamp in seconds since the Unix epoch
        #[arg(help = "The Unix timestamp to convert")]
        timestamp: i64,

        /// The target timezone (e.g., "America/New_York")
        #[arg(help = "The target timezone, e.g., America/New_York")]
        timezone: String,
    },

    /// Convert a Unix timestamp to a human-readable format
    #[command(aliases = ["ctr"], long_about = "Convert a Unix timestamp to the system's local time in a human-readable format.")]
    Ctr {
        /// The timestamp in seconds since the Unix epoch
        #[arg(help = "The Unix timestamp to convert")]
        timestamp: i64,
    },

    /// Get the current Unix timestamp
    #[command(aliases = ["ct"], long_about = "Fetch the current Unix timestamp (seconds since epoch).")]
    Ct,
}

fn main() {
    let args = Cli::parse();

    if let Some(command) = args.command {
        match command {
            Commands::Ctz {
                timestamp,
                timezone,
            } => match timezone.parse::<Tz>() {
                Ok(tz) => {
                    let dt = Utc.timestamp_opt(timestamp, 0).unwrap();
                    let localized = dt.with_timezone(&tz);
                    println!("\n{}\n", localized);
                }
                Err(_) => eprintln!("\nInvalid timezone: {}\n", timezone),
            },
            Commands::Ctr { timestamp } => {
                let dt = NaiveDateTime::from_timestamp_opt(timestamp, 0);
                if let Some(dt) = dt {
                    let local: DateTime<Local> = Local.from_utc_datetime(&dt);
                    println!("\n{}\n", local.format("%Y-%m-%d %H:%M:%S"));
                } else {
                    eprintln!("\nInvalid timestamp: {}\n", timestamp);
                }
            }
            Commands::Ct => {
                let now = Utc::now();
                println!("\n{}\n", now.timestamp());
            }
        }
    } else if let Some(expression) = args.expression {
        // Default behavior: Evaluate a mathematical expression
        match eval_str(&expression) {
            Ok(result) => println!("\n{}\n", result),
            Err(e) => eprintln!("\nError: {}\n", e),
        }
    } else {
        eprintln!("\nNo input provided. Use '--help' for usage information.\n");
    }
}
