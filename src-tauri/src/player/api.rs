use std::sync::{Mutex, Arc};

use librespot::core::authentication::Credentials;
use librespot::core::cache::Cache;
use librespot::core::config::SessionConfig;
use librespot::core::session::Session;
use librespot::core::spotify_id::{SpotifyId, SpotifyItemType};
use librespot::playback::audio_backend;
use librespot::playback::config::{AudioFormat, Bitrate, PlayerConfig, VolumeCtrl};
use librespot::playback::mixer::softmixer::SoftMixer;
use librespot::playback::mixer::{MixerConfig, Mixer};
use librespot::playback::player::{Player, PlayerEventChannel, PlayerEvent};
use librespot::core::error::ErrorKind as SessionErrorKind;
use tauri::{Manager, Window};

use crate::{PLAYER_STATE, USER_SESSION, ACCESS_TOKEN, AccessToken};
use crate::player::models::{Volume, PlayerState, PlayerEventMessage, self};

use super::models::{PlayerEventData, SessionConnectionError};

#[tauri::command]
pub fn load_track(id: String) -> Result<(), ()>
{
    let guard = PLAYER_STATE.lock().unwrap();
    let mut player = guard.as_ref().unwrap().lock().unwrap();

    let mut track = SpotifyId::from_base62(&id).unwrap();
    track.item_type = SpotifyItemType::Track;

    player.load(track);

    Ok(())
}

#[tauri::command]
pub fn play_track() -> Result<(), ()>
{
    let guard = PLAYER_STATE.lock().unwrap();
    let mut player = guard.as_ref().unwrap().lock().unwrap();

    player.play();

    Ok(())
}

#[tauri::command]
pub fn pause_track()
{
    let guard = PLAYER_STATE.lock().unwrap();
    let mut player = guard.as_ref().unwrap().lock().unwrap();

    player.pause();
}

#[tauri::command]
pub fn set_volume(volume: u16){
    let guard = PLAYER_STATE.lock().unwrap();
    let mut player = guard.as_ref().unwrap().lock().unwrap();

    player.set_volume(volume);
}

#[tauri::command]
pub fn set_track_position(position: u32)
{
    let guard = PLAYER_STATE.lock().unwrap();
    let mut player = guard.as_ref().unwrap().lock().unwrap();

    player.set_position(position);
}


pub async fn start_player(win: Window)
{
    let session = match USER_SESSION.lock().unwrap().as_ref() {
        Some(session) => session.clone(),
        None => panic!("No session found")
    };

    // spotify:track:6JzzI3YxHCcjZ7MCQS2YS1
    let player_config = PlayerConfig {
        bitrate: Bitrate::Bitrate160,
        gapless: false,
        passthrough: false,
        normalisation: false,
        normalisation_pregain_db: 0.0,
        normalisation_threshold_dbfs: 0.0,
        normalisation_attack_cf: 0.0,
        normalisation_release_cf: 0.0,
        normalisation_knee_db: 0.0,
        ditherer: None,
        ..Default::default()
    };
    let audio_format = AudioFormat::default();
    let backend = audio_backend::find(None).unwrap();
    let mix = Box::new(SoftMixer::open(MixerConfig {
        volume_ctrl: VolumeCtrl::Linear,
        ..Default::default()
    }));
    
    let player = Player::new(player_config, session, mix.get_soft_volume(), move || {
        backend(None, audio_format)
    });

    tokio::spawn(start_player_channel(player.get_player_event_channel(), win));
    let player_arc = Arc::new(Mutex::new(player));

    println!("Player started");
    
    *PLAYER_STATE.lock().unwrap() = Some(Mutex::new(PlayerState{
        player: player_arc,
        volume: Volume::new(mix)
    }));
}

async fn start_player_channel(mut player_channel: PlayerEventChannel, win: Window) -> ! {
    loop {
        let event = match player_channel.recv().await {
            Some(event) => event,
            None => continue,  
        };

        match event {
            PlayerEvent::VolumeChanged { volume } => {
                println!("Volume changed to {}", volume);
            },
            PlayerEvent::PositionCorrection { play_request_id, position_ms, track_id } => {
                println!("Position correction for play request {:?} at position {} for track {:?}", play_request_id, position_ms, track_id);
            },
            PlayerEvent::Stopped { .. } => {
                println!("Stopped");
            },
            PlayerEvent::EndOfTrack { .. } => {
                println!("End of track");
            },
            PlayerEvent::Loading { play_request_id, track_id, position_ms } => {
                println!("Loading track {:?} for play request {:?} at position {}", track_id, play_request_id, position_ms);
            },
            PlayerEvent::Preloading { track_id } => {
                println!("Preloading track {:?}", track_id);
            },
            PlayerEvent::Playing { play_request_id: _, position_ms, track_id} => {
                win.emit_all("player://client_handler", PlayerEventMessage {
                    event: models::PlayerEvents::Playing,
                    data: Some( PlayerEventData {
                        position_ms: position_ms,
                        track_id: track_id.to_base62().unwrap(),
                    }),
                }).unwrap();
            },
            PlayerEvent::Paused { play_request_id, position_ms, track_id } => {
                println!("paused");
                win.emit_all("player://client_handler", PlayerEventMessage {
                    event: models::PlayerEvents::Paused,
                    data: Some( PlayerEventData {
                        position_ms: position_ms,
                        track_id: track_id.to_base62().unwrap(),
                    }),
                }).unwrap();
            },
            PlayerEvent::Stopped { play_request_id, track_id: _} => {
                println!("Stopped at {}", play_request_id);
            },
            PlayerEvent::TimeToPreloadNextTrack { track_id, play_request_id: _ } => {
                println!("Time to preload next track {:?}", track_id);
            },
            PlayerEvent::Unavailable { play_request_id:_, track_id } => {
                println!("Unavailable track {:?}", track_id);
            },
            PlayerEvent::Seeked { play_request_id, track_id, position_ms } => {

            },
            PlayerEvent::TrackChanged { audio_item } => {
                
            },
            PlayerEvent::SessionConnected { connection_id, user_name } => todo!(),
            PlayerEvent::SessionDisconnected { connection_id, user_name } => todo!(),
            PlayerEvent::SessionClientChanged { client_id, client_name, client_brand_name, client_model_name } => todo!(),
            PlayerEvent::ShuffleChanged { shuffle } => todo!(),
            PlayerEvent::RepeatChanged { repeat } => todo!(),
            PlayerEvent::AutoPlayChanged { auto_play } => todo!(),
            PlayerEvent::FilterExplicitContentChanged { filter } => todo!(),
        }
        
    }
}

#[tauri::command]
pub async fn login_session(
    email: Option<String>, 
    password: Option<String>, 
    window: Window
) -> Result<AccessToken, SessionConnectionError>
{
    let scopes = "user-read-private,user-read-email,playlist-read-private,user-modify-playback-state,streaming";
    let session_config = SessionConfig::default();
    let cache_config = Cache::new(Some(dirs::document_dir().unwrap().join("spotify-lite")), None, None, None).unwrap();
    let credentials = match cache_config.credentials() {
        Some(credentials) => credentials,
        None => Credentials::with_password(email.unwrap(), password.unwrap())
    };

    let session = Session::new(session_config, Some(cache_config));

    if let Err(err) = session.connect(credentials, true).await {
        match err.kind {
            SessionErrorKind::Unauthenticated => 
                return Err(SessionConnectionError::InvalidCredentials),
            _ =>
                return Err(SessionConnectionError::UnknownError)
        }
    };

    let token = match session.token_provider().get_token(scopes).await {
        Ok(token) => token,
        Err(err) => {
            dbg!(err);
            return Err(SessionConnectionError::UnableToGetAccessToken);
        }
    };
    let access = AccessToken {
        access_token: token.access_token,
        expires_in: token.expires_in,
        token_type: token.token_type,
    };

    *ACCESS_TOKEN.lock().unwrap() = Some(access.clone());
    *USER_SESSION.lock().unwrap() = Some(session);

    tokio::spawn(start_player(window));

    Ok(access)
}