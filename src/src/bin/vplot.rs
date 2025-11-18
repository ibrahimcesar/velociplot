//! velociplot CLI - Quick plotting from the command line

use clap::{Parser, Subcommand};

/// velociplot CLI - Scientific plotting at velociraptor speed 🦖
#[derive(Parser)]
#[command(name = "vplot")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Plot data from file
    Plot {
        /// Input data file (CSV, JSON)
        #[arg(value_name = "FILE")]
        input: String,
        
        /// Plot type
        #[arg(short, long, value_enum, default_value = "line")]
        plot_type: PlotType,
        
        /// X-axis column
        #[arg(short = 'x', long)]
        x_column: Option<String>,
        
        /// Y-axis column(s)
        #[arg(short = 'y', long)]
        y_column: Vec<String>,
        
        /// Output file
        #[arg(short, long, default_value = "plot.png")]
        output: String,
    },
    
    /// Show version info
    Version,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum PlotType {
    Line,
    Scatter,
    Bar,
    Histogram,
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Plot { input, plot_type, x_column, y_column, output } => {
            println!("🦖 velociplot - Scientific plotting at velociraptor speed");
            println!();
            println!("Input:  {}", input);
            println!("Type:   {:?}", plot_type);
            println!("X:      {:?}", x_column);
            println!("Y:      {:?}", y_column);
            println!("Output: {}", output);
            println!();
            println!("🚧 Implementation coming soon...");
        }
        Commands::Version => {
            println!("velociplot v{}", env!("CARGO_PKG_VERSION"));
            println!("Fast, publication-quality scientific plotting");
        }
    }
}
