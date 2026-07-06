use clap::Parser;
use lofty;
use lofty::config::WriteOptions;
use lofty::file::{AudioFile, TaggedFile, TaggedFileExt};
use lofty::read_from_path;
use lofty::tag::{ItemKey, TagType};
use walkdir::WalkDir;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Use when indicating a directory to 'mass scan'
    #[arg(short, long)]
    directory: Option<String>,
    /// Use when indicating a singular file, relative pathing
    #[arg(short, long)]
    file: Option<String>,
}
// this thing is being used wtf?!
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
    .filter_map(|k| ItemKey::from_key(TagType::VorbisComments, k))
    .collect();

    let mut result = ParseResult::Skipped;

    for tag_type in [TagType::VorbisComments] {
        // since unfortunately, f.tags_mut() doesnt exist, i gotta do it the manual way

        if let Some(tag) = file.tag_mut(tag_type) {
            // TODO: actually... strip the fucking tags?
            for key in &fuck_you_murder_list {
                println!(
                    "Removing {key:?} from {}",
                    tag.get_string(ItemKey::TrackTitle)?
                );
                tag.remove_key(*key)
            }
            result = ParseResult::Handled;
        }
    }

    Some(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let music_args = Args::parse();
    let directory = music_args.directory;
    let file = music_args.file;
    println!("{directory:?}");
    println!("{file:?}");
    match (directory, file) {
        (Some(dir), None) => {
            for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    let path = entry.path();
                    if path.extension().and_then(|e| e.to_str()) == Some("flac") {
                    // TODO: file processing but im already kinda balls deep into this and im almost done! so! little break time!
                    }
                }
            }
        }
        (None, Some(f)) => {
            println!("Scanning File!");
            let mut file = read_from_path(&f)?;
            parse_file(&mut file);
            file.save_to_path(f, WriteOptions::default())?;
            println!("Success!")
        }
        (None, None) => {
            eprintln!("gotta pass either --directory or --file");
            std::process::exit(1);
        }
        (Some(_), Some(_)) => {
            eprintln!("One or the other, please.");
            std::process::exit(1)
        }
    }
    Ok(())
}
