#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use std::{sync::Mutex, time::Duration};
use librespot::core::Session;
use tauri::Manager;

mod spotify_api;
mod player;

lazy_static! {
    pub static ref PLAYER_STATE: Mutex<Option<Mutex<player::models::PlayerState>>> = Mutex::new(None);
    pub static ref ACCESS_TOKEN: Mutex<Option<AccessToken>> = Mutex::new(None);
    pub static ref USER_SESSION: Mutex<Option<Session>> = Mutex::new(None);
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessToken {
    access_token: String,
    token_type: String,
    expires_in: Duration,
}

#[tokio::main]
async fn main() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .setup(move |app| {
            return Ok(());
        })
        .invoke_handler(tauri::generate_handler![
            spotify_api::client::get_playlist,
            spotify_api::client::get_playlist_tracks,
            spotify_api::client::validate_access_token,
            player::api::play_track,
            player::api::pause_track,
            player::api::load_track,
            player::api::set_volume,
            player::api::set_track_position,
            player::api::login_session
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}