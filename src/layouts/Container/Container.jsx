import React from 'react';

import Playlist from '../Playlist/Playlist';

import './style.css';

function Container({currentPlaylist, setTrack}) {
    return (
        <div className="container-layout">
            <Playlist currentPlaylist={currentPlaylist}  />
        </div>
    );
}

export default Container;