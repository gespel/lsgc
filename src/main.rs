
use std::io::Write;
use serde_json::{Error, Value};
use log::{error, info};
use crate::riftcrawler::RiftCrawler;
mod riftcrawler;
mod tools;


#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    info!("Setting up file structure now...");
    tools::setup_folder("games".to_string());
    tools::setup_folder("games/classic".to_string());
    tools::setup_folder("games/aram".to_string());
    info!("File structure done!");

    let mut l = RiftCrawler::new("RGAPI-a7573798-3d74-48d6-9b9b-17c347e3b5e8".parse().unwrap());
    info!("Starting initial Playerfetch");
    l.get_games_from_player("TFO Gespel").await.expect("TODO: panic message");
    l.write_games_to_disk_and_extract_new_players().await.expect("");
    println!("{:?}", l.games_list);
    /*loop {
        info!("New Epoch!");

        info!("Fetching games from players...");
        l.get_games_from_players(5).await.expect("TODO: panic message");

        info!("Writing to disk and extracting new players..");
        l.write_games_to_disk_and_extract_new_players().await.expect("TODO: panic message");

    }*/



    Ok(())
}

