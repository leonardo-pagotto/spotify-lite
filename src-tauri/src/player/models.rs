use std::sync::{ Arc, Mutex };
use librespot::{
    core::{
        session::Session,
        authentication::Credentials,
        spotify_id::SpotifyId
    },
    playback::{
        mixer::{
            softmixer::SoftMixer,
            Mixer
        },
        player::Player
    },
};
use serde::{Serialize, Deserialize};

pub struct PlayerState {
    pub player: Arc<Mutex<Player>>,
    pub volume: Volume,
}

pub struct Volume {
    pub current: u16,
    pub max: u16,
    pub controller: Box<SoftMixer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerEvents {
    Playing,
    Paused,
    Stopped,
    EndOfTrack,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerEventMessage {
    pub event: PlayerEvents,
    pub data: Option<PlayerEventData>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerEventData{
    pub track_id: String,
    pub position_ms: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SessionConnectionError {
    InvalidCredentials,
    UnknownError,
    InvalidCacheCredentials,
    UnableToGetAccessToken,
}

// INPLEMENTATIONS
impl Volume {
    pub fn new(controller: Box<SoftMixer>) -> Volume {
        Volume {
            max: controller.volume(),
            current: 0,
            controller,
        }
    }
}

impl PlayerState{
    pub fn new(player: Arc<Mutex<Player>>, volume: Volume) -> PlayerState {
        PlayerState {
            player,
            volume,
        }
    }

    pub fn pause(&mut self) {
        self.player.lock().unwrap().pause();
    }

    pub fn stop(&mut self) {
        self.player.lock().unwrap().stop();
    }

    pub fn play(&mut self) {
        self.player.lock().unwrap().play();
        self.player.lock().unwrap();
    }

    pub fn load(&mut self, track_id: SpotifyId) {
        self.player.lock().unwrap().load(track_id, true, 0);
    }

    pub fn set_volume(&mut self, volume: u16) {
        self.volume.current = volume;
        self.volume.controller.set_volume((self.volume.max / 100) * volume);
    }

    pub fn set_position(&mut self, position_ms: u32) {
        self.player.lock().unwrap().seek(position_ms);
    }
}