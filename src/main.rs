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
        Some("next")   => client.next().unwrap(),
        Some("pause")  => client.pause(true).unwrap(),
        Some("play")   => client.play().unwrap(),
        Some("prev")   => client.prev().unwrap(),
        Some("stop")   => client.stop().unwrap(),
        Some("toggle") => toggle(client, status),

        // State Changers
        Some("consume") => consume(client, status),
        Some("random")  => random(client, status),
        Some("repeat")  => repeat(client, status),
        Some("single")  => single(client, status),

        // Playlist Functions
        Some("shuffle") => client.shuffle(..).unwrap(),

        Some("version") => version(client),

        // For commands not accounted for
        _ => {},
    }
}
