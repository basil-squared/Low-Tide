use lofty;
use lofty::ogg::VorbisComments;
use lofty::read_from_path;
use clap::Parser;
use lofty::file::{TaggedFile, TaggedFileExt};
use lofty::tag::{ItemKey, Tag, TagType};
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

enum ParseResult {
    Handled,
    Skipped,
}
fn parse_file(file: &mut TaggedFile) -> Option<ParseResult> {
    // this is my vector list, say hi
    let fuck_you_murder_list: Vec<ItemKey> = vec![
        "TIDAL_ALBUM_ID",
        "TIDAL_ALBUM_URL",
        "TIDAL_DATA",
        "TIDAL_TRACK_ID",
        "TIDAL_TRACK_URL",
    
    ]
    .into_iter()
    .filter_map(|k| ItemKey::from_key(TagType::Id3v2, k))
    .collect();

    
    let mut result = ParseResult::Skipped;

    for tag_type in [TagType::VorbisComments, TagType::Id3v2] {
        // since unfortunately, f.tags_mut() doesnt exist, i gotta do it the manual way
        if let Some(tag) = file.tag_mut(tag_type) {
            // TODO: actually... strip the fucking tags?
            result = ParseResult::Handled;
        }
    }

    Some(result)
}

fn main() {
    let music_args = Args::parse();
    let directory = music_args.directory;
    let file = music_args.file;
    println!("{directory:?}");
    println!("{file:?}");
    match (directory,file) {
        (Some(dir),None) => {

        }
        (None, Some(f)) => {
            println!("Scanning File!");


        }
        (None, None) => {
            eprintln!("gotta pass either --directory or --file");
            std::process::exit(1);

        }
        (Some(_),Some(_)) => {
            eprintln!("One or the other, please.");
            std::process::exit(1)

        }

    }

    
}


