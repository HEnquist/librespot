extern crate log;
extern crate env_logger;

extern crate librespot;
extern crate tokio_core;
//extern crate tokio_fs;
extern crate tokio_io;
extern crate futures;
//extern crate futures_cpupool;

use std::env;
use tokio_core::reactor::Core;

use librespot::core::authentication::Credentials;
use librespot::core::config::SessionConfig;
use librespot::core::session::Session;
use librespot::core::spotify_id::SpotifyId;
use librespot::metadata::{Metadata, Track, JsAlbum, JsonMeta};

fn main() {
    env_logger::init();
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let session_config = SessionConfig::default();

    let args: Vec<_> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: {} USERNAME PASSWORD ALBUM", args[0]);
    }
    let username = args[1].to_owned();
    let password = args[2].to_owned();
    let credentials = Credentials::with_password(username, password);

    let uri_split = args[3].split(":");
    let uri_parts: Vec<&str> = uri_split.collect();
    println!("{}, {}, {}",uri_parts[0], uri_parts[1], uri_parts[2]);
    
    let album_uri = SpotifyId::from_base62(uri_parts[2]).unwrap();
    
    let session = core
        .run(Session::connect(session_config, credentials, None, handle))
        .unwrap();

    let album = core.run(JsAlbum::jget(&session, album_uri)).unwrap();
    println!("{:?}",album);
    //for track_id in album.tracks {
    //    let album_track = core.run(Track::get(&session, track_id)).unwrap();
    //    println!("track: {} ", album_track.name);
    //}
}
