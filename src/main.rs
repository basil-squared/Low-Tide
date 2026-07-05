use lofty;
use clap::Parser;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// Use when indicating a directory to 'mass scan'
 #[arg(short,long)]
 directory: Option<String>,
 /// Use when indicating a singular file, relative pathing
 #[arg(short,long)]
 file: Option<String>,

}


fn main() {
    let args = Args::parse();
}


