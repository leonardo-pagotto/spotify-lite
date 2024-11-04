import React, {useCallback, useEffect, useState} from 'react';
import { invoke } from '@tauri-apps/api';

import PlaylistCard from '../../components/PlaylistCard/PlaylistCard';

import './style.css'

function Navbar({setPlaylist}) {

    const [playlists, setPlaylists] = useState();

    const getPlaylists = useCallback(() => {
        const accessToken = localStorage.getItem("accessToken");

        invoke("get_playlist", {accessToken})
            .then((result) => 
                setPlaylists({items: result.items, callback: showPlaylist}))
                .catch((err) => console.error(err));
    })

    async function showPlaylist(playlistId)
    {
        let result = await invoke("get_playlist_tracks", {
            playlistId,
            offset: 0,
            limit: 30,
            accessToken: localStorage.getItem("accessToken")
        });
        
        const tracks = result?.items.map(i => {
            return i.track;
        })
        
        const playlist = {
            id: playlistId, 
            tracks
        }

        setPlaylist((prev) => playlist);
    }

    useEffect(() => {
        getPlaylists();
    }, []);

    return (
        <div className='navbar'>
            <div className='routes'>
                <span><span className="material-icons">home_outline</span> Home</span>
                <span style={{marginTop: "20px"}}><span className="material-icons">search_outline</span> Search</span>
            </div>
            <span className='title'><span className="material-icons">library_music</span>Your Library</span>
            <div className='user-area'>
                {playlists?.items.map(playlist => {
                    return (
                        <div className='opt' onClick={() => playlists.callback(playlist.id)}>
                            <PlaylistCard key={playlist.id} name={playlist.name} owner={playlist.owner.display_name} image={playlist.images[0].url} />
                        </div>
                    )
                })}
            </div>
        </div>
    );
}

export default Navbar;