extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

mod simfile;
mod simfile_parser;

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
    let groups = try!(fs::read_dir(path));

    for group_listing in groups {
        let group = try!(group_listing);

        try!(parse_group_folder(group));
    }

    return Ok(());
}

fn parse_group_folder(group: DirEntry) -> io::Result<()> {
    let group_metadata = try!(group.metadata());
    if !group_metadata.is_dir() {
        return Ok(());
    }

    let songs = try!(fs::read_dir(group.path()));

    for song_listing in songs {
        let song = try!(song_listing);

        parse_song_folder(song);
    }

    return Ok(());
}

fn parse_song_folder(song: DirEntry) {
    println!("{:?}", song.path());
}

