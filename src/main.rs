// External crates
#[macro_use]
extern crate clap;
extern crate mpd;

use clap::App;
use mpd::client::Client;

// Internal modules
pub mod functions;

use functions::*;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let args = App::from_yaml(yaml).get_matches();

    // Setup MPD connection
    let host = args.value_of("host").unwrap_or("127.0.0.1");
    let port = args.value_of("port").unwrap_or("6600");
    let mpd = format!("{}:{}", host, port);

    let mut client = Client::connect(&mpd)
                            .expect("Unable to open connection to MPD");

    let status = client.status().unwrap();

    match args.subcommand_name() {
        // Playback Controls
        Some("next")   => client.next().unwrap(),
        Some("pause")  => client.pause(true).unwrap(),
        Some("play")   => client.play().unwrap(),
        Some("prev")   => client.prev().unwrap(),
        Some("stop")   => client.stop().unwrap(),
        Some("toggle") => toggle(&mut client, &status),

        // State Changers
        Some("consume") => consume(&mut client, &status),
        Some("random")  => random(&mut client, &status),
        Some("repeat")  => repeat(&mut client, &status),
        Some("single")  => single(&mut client, &status),

        // Playlist Functions
        Some("clear")     => client.clear().unwrap(),
        Some("playlists") => playlists(&mut client),
        Some("shuffle")   => client.shuffle(..).unwrap(),

        // Misc
        Some("status")  => {}, // Will still call mpd_status below
        Some("version") => version(&mut client),

        _ => {},
    };

    if !args.is_present("quiet") {
        mpd_status(&mut client)
    }
}
