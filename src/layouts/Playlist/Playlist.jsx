import React, { useEffect } from 'react';

import TrackCard from '../../components/TrakerCard/TrackCard';

import './style.css';

function Playlist({currentPlaylist}) {

    useEffect(() => {
        console.log(currentPlaylist);
    });

    return (
        <div className='playlist'>
            <div className='header'>
                <div className='informations'>
                    <img src={currentPlaylist} alt="" />
                </div>
                <div className='controllers'>

                </div>
            </div>
            <div className='content'>
                {currentPlaylist?.tracks?.map((track, position) => {
                    return <TrackCard
                        position={position + 1}
                        key={track.id}
                        id={track.id}
                        name={track.name} 
                        image={track.album.images[0].url} 
                        artist={track.album.artists[0].name} 
                        album={track.album.name} 
                        duration={track.duration_ms}
                    />
                })}
            </div>
        </div>
    );
}

export default Playlist;