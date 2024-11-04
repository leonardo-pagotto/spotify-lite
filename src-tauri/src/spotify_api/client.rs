use reqwest::header::AUTHORIZATION;

use crate::spotify_api::api_models::*;

#[tauri::command]
pub async fn get_playlist(access_token:&str) -> Result<Page<Playlist>, String>
{
    let req = reqwest::Client::new()
        .get("https://api.spotify.com/v1/me/playlists")
        .header(AUTHORIZATION, format!("Bearer {}", access_token))
    ;

    let response = match req.send().await {
        Ok(response) => response,
        Err(_) => return Err("Failed to get playlists".to_string()),
    };

    if response.status().is_success() {
        return Ok(response.json::<Page<Playlist>>().await.unwrap());
    }

    Err("Failed to get playlists".to_string())

}

#[tauri::command]
pub async fn get_playlist_tracks(access_token:&str,  playlist_id:&str, offset: usize, limit: usize,) -> Result<Page<PlaylistTrack>, String>
{
    let query_params = [
        ("limit", limit.to_string()),
        ("offset", offset.to_string()),
    ];

    let req = reqwest::Client::new()
        .get(format!("https://api.spotify.com/v1/playlists/{}/tracks", playlist_id))
        .query(&query_params)
        .header(AUTHORIZATION, format!("Bearer {}", access_token))
    ;

    let res = req.send().await.unwrap();

    Ok(res.json::<Page<PlaylistTrack>>().await.unwrap())
}

#[tauri::command]
pub async fn validate_access_token(access_token:&str) -> Result<(), ()>
{
    let req = reqwest::Client::new()
        .get(format!("https://api.spotify.com/v1/me "))
        .header(AUTHORIZATION, format!("Bearer {}", access_token))
    ;

    let res = req.send().await.unwrap();

    if !res.status().is_success()
    {
        return Err(());
    }

    Ok(())
}