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

    // Match subcommands with arguments
    match args.subcommand() {
        ("volume", Some(args)) => set_volume(&mut client, args.value_of("v")),
        _ => {}
    }

    // Match subcommands with no arguments
    match args.subcommand_name() {
        // Playback Controls
        Some("next")   => client.next().unwrap(),
        Some("pause")  => client.pause(true).unwrap(),
        Some("play")   => client.play().unwrap(),
        Some("prev")   => client.prev().unwrap(),
        Some("stop")   => client.stop().unwrap(),
        Some("toggle") => toggle(&mut client),

        // State Changers
        Some("consume") => consume(&mut client),
        Some("random")  => random(&mut client),
        Some("repeat")  => repeat(&mut client),
        Some("single")  => single(&mut client),

        // Playlist Functions
        Some("clear")     => client.clear().unwrap(),
        Some("playlists") => playlists(&mut client),
        Some("shuffle")   => client.shuffle(..).unwrap(),

        // Misc
        Some("current") => current(&mut client),
        Some("status")  => {}, // Will still call mpd_status below
        Some("version") => version(&mut client),

        _ => {},
    };

    // Determine whether to print status or not
    let curr_present = args.subcommand_matches("current").is_some();
    let quiet_present = args.is_present("quiet");

    if curr_present || quiet_present {} else {
        mpd_status(&mut client)
    }
}
