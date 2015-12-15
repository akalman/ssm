extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

const USAGE: &'static str =
"
Stepmania Song Manager

Usage:
  ssm init
  ssm build
  ssm (-h | --help)
  ssm --version

Options:
  -h --help  Show this screen.
  --version  Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_init: bool,
    cmd_build: bool,
}


fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    let result = parse_songs_folder();
    match result.err() {
        Some(x) => println!("{:?}", x),
	None => println!("done"),
    }
}

fn parse_songs_folder() -> io::Result<()> {
    let path = Path::new("./Songs");

    for group in try!(fs::read_dir(path)) {
        try!(parse_group_folder(try!(group)));
    }

    Ok(())
}

fn parse_group_folder(group: DirEntry) -> io::Result<()> {
    for song in try!(fs::read_dir(group.path())) {
        parse_song_folder(try!(song));
    }

    Ok(())
}

fn parse_song_folder(song: DirEntry) {
    println!("{:?}", song.path());
}

