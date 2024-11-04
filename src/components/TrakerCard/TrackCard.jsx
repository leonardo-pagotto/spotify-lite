import React, { useEffect, useState } from 'react';
import { emit } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/tauri'
import { playerEnventsTypes } from '../../services/player';

import './style.css';

function TrackCard({position, id, name, image, artist, album, duration}) {
    
    function transformTime()
    {
        const totalSeconds = Math.floor(duration / 1000);
        const hours = Math.floor(totalSeconds / 3600);
        const minutes = Math.floor((totalSeconds % 3600) / 60);
        const seconds = (totalSeconds % 60).toString().padStart(2, '0');

        if (hours > 0) {
            return `${hours}:${minutes}:${seconds}`;
        }
        
        return `${minutes}:${seconds}`;
    }

    function playTrack()
    {
        invoke('load_track', { id });
        emit('player://client_handler', {
            event: playerEnventsTypes.Load, 
            data: {id, name, image, artist, album, duration} 
        });
    }

    return (
        <div className='track_card' onClick={playTrack}>
            <div className='about'>
                <span className='position'>{position}</span>
                <img className='image' src={image} alt="" />
                <div className='music_data'>
                    <span className='name'>{name}</span>
                    <span className='artist'>{artist}</span>
                </div>
            </div>
            <span className='album'>{album}</span>
            <span className='duration'>{transformTime()}</span>
        </div>
    );
}

export default TrackCard;