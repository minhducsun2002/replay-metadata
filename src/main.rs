use core::panic;
use osu_db::replay::Replay;
use std::{env, fs::File, io::Read, vec::Vec};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct JsonReplay {
    pub mode: u32,
    pub version: u32,
    pub beatmap_hash: String,
    pub player_name: String,
    pub replay_hash: String,
    pub count_300: u16, pub count_100: u16, pub count_50: u16,
    pub count_geki: u16, pub count_katsu: u16, pub count_miss: u16,
    pub score: u32, pub max_combo: u16, pub perfect_combo: bool,
    pub mods: u32,
    pub timestamp: String,
}

fn main() {
    let _path = env::args_os().nth(1);
    if _path == None { panic!("No replay file was passed!"); }
    let path = _path.unwrap();
    let mut file = File::open(path.as_os_str()).unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    if file.read_to_end(&mut buffer).is_err() { panic!("Failed to read file {:?}", path); }
    let _replay = Replay::from_bytes(&buffer);
    if _replay.is_err() { panic!("Failed to parse replay from file {:?}", path); }
    let replay = _replay.unwrap();

    let json_replay = JsonReplay {
        mode: replay.mode as u32,
        version: replay.version,
        beatmap_hash: replay.beatmap_hash.unwrap(),
        player_name: replay.player_name.unwrap(),
        replay_hash: replay.replay_hash.unwrap(),

        count_300: replay.count_300,
        count_100: replay.count_100,
        count_50: replay.count_50,

        count_geki: replay.count_geki,
        count_katsu: replay.count_katsu,
        count_miss: replay.count_miss,

        score: replay.score, max_combo: replay.max_combo, perfect_combo: replay.perfect_combo,

        mods: replay.mods.bits(), timestamp: replay.timestamp.to_rfc3339() + "Z"
    };
    println!("{}", serde_json::to_string(&json_replay).unwrap());
}
