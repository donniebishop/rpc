/// Module for helper functions to rpc

// External
use mpd::client::Client;
use mpd::status::Status;
use mpd::status::State::*;

// STD
use std::ops::Rem;

//------------------
// Playback Controls
//------------------

pub fn toggle(c: &mut Client, s: &Status) {
    // Invert current pause status
    match s.state {
       Pause => c.pause(false).unwrap(),
       Play => c.pause(true).unwrap(),
       Stop => c.play().unwrap()
    }
}

//---------------
// State Changers
//---------------

pub fn consume(c: &mut Client, s: &Status) {
    c.consume(!s.consume).unwrap()
}

pub fn random(c: &mut Client, s: &Status) {
    c.random(!s.random).unwrap()
}

pub fn repeat(c: &mut Client, s: &Status) {
    c.repeat(!s.repeat).unwrap()
}

pub fn single(c: &mut Client, s: &Status) {
    c.single(!s.single).unwrap()

    //if b {
        //c.single(true).unwrap() 
    //} else {
        //c.single(false).unwrap()
    //}
}

//-------------------
// Playlist Functions
//-------------------

pub fn playlists(c: &mut Client) {
    let ps = c.playlists().unwrap();

    for p in ps {
        println!("{}", p.name)
    }
}

//------------------------
// Miscellaneous Functions
//------------------------

pub fn mpd_status(c: &mut Client) {
    // Get current Status
    let s = c.status().unwrap();

    // First Line
    let song = c.currentsong().unwrap().unwrap();
    let title = song.title.unwrap();
    let artist = song.tags.get("Artist").unwrap();

    // Second Line
    let state = match s.state {
        Pause => "paused",
        Play => "playing",
        Stop => "stopped",
    };

    // Wow. This was a nightmare to do. I'm so sorry
    let t = s.time.unwrap();
    let (e, d) = (t.0, t.1);
    let elapsed = if e.num_seconds().rem(60) < 10 {
        format!("{}:0{}", e.num_minutes(), 
                          (e.num_seconds() - (e.num_minutes() * 60)))
    } else {
        format!("{}:{}", e.num_minutes(), 
                          (e.num_seconds() - (e.num_minutes() * 60)))
    };
    let duration = if d.num_seconds().rem(60) < 10 {
        format!("{}:0{}", d.num_minutes(), 
                          (d.num_seconds() - (d.num_minutes() * 60)))
    } else {
        format!("{}:{}", d.num_minutes(), 
                          (d.num_seconds() - (d.num_minutes() * 60)))
    };

    // Third Line
    let onoff = |b| if b { "on" } else { "off" };
    let volume = s.volume;
    let repeat = onoff(s.repeat);
    let random = onoff(s.random);
    let single = onoff(s.single);
    let consume = onoff(s.consume);

    println!("{} - {}", artist, title);
    println!("[{}] {}/{}", state, elapsed, duration);
    println!("volume:{}%  repeat:{}  random:{}  single:{}  consume:{}", volume,
                                                                        repeat,
                                                                        random,
                                                                        single,
                                                                        consume)
}

pub fn version(c: &mut Client) {
    let v = c.version;
    let (major, minor, patch) = (v.0, v.1, v.2);
    println!("mpd version: {}.{}.{}", major, minor, patch)
}
