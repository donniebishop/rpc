/// Module for helper functions to rpc

// External
use mpd::client::Client;
use mpd::status::Status;
use mpd::status::State::*;

pub fn toggle(mut c: Client, s: Status) {
    // Invert current pause status
    match s.state {
       Pause => c.pause(false).unwrap(),
       Play => c.pause(true).unwrap(),
       Stop => c.play().unwrap()
    }
}

pub fn consume(mut c: Client, s: Status) {
    c.consume(!s.consume).unwrap()
}

pub fn random(mut c: Client, s: Status) {
    c.random(!s.random).unwrap()
}

pub fn repeat(mut c: Client, s: Status) {
    c.repeat(!s.repeat).unwrap()
}

pub fn single(mut c: Client, s: Status) {
    c.single(!s.single).unwrap()

    //if b {
        //c.single(true).unwrap() 
    //} else {
        //c.single(false).unwrap()
    //}
}

pub fn version(c: Client) {
    let v = c.version;
    let (major, minor, patch) = (v.0, v.1, v.2);
    println!("mpd version: {}.{}.{}", major, minor, patch)
}
