import React, { useCallback, useEffect, useImperativeHandle, useRef, useState } from 'react';
import * as tauriEvent from '@tauri-apps/api/event';
import * as player from "../../services/player.js";
import { listen } from '@tauri-apps/api/event';

import './style.css';
import music_cape from '../../assets/examples/music_cape.jpg';

function Musicbar() {

    const playOptions = {
        play: "play_circle_filled",
        pause: "pause_circle_filled"
    }

    const [ playBtn, setPlayBtn ] = React.useState(playOptions.play);
    const [track, setTrack] = useState();
    const trackRefData = useRef(null);
    const intervalTrackBar = useRef(null);
    const trackPosition = useRef(0);

    const updateTrackPosition = useCallback(() => {
        const durationMs = trackRefData.current.duration;
        const positionMs = trackRefData.current.position;
        
        const progressBar = document.getElementById('track-bar-current'); 
        const intervalMs = 1000; // Intervalo em milissegundos (1 segundo)
        const maxProgress = 100; // Valor máximo do input range
        const progressIncrement = maxProgress / (durationMs / intervalMs); // Incremento do progresso 
        trackPosition.current = (positionMs / durationMs) * 100; // Calcula a posição atual da música em porcentagem
        
        let isUserInteracting = false;

        clearInterval(intervalTrackBar.current);

        intervalTrackBar.current = setInterval(() => {
            if (trackPosition.current >= maxProgress) {
                clearInterval(intervalTrackBar.current);
                trackPosition.current = maxProgress;
                setPlayBtn(playOptions.play);
            }
            
            if(isUserInteracting) return;
            
            trackPosition.current += progressIncrement;
            progressBar.value = trackPosition.current;
        }, intervalMs);

        progressBar.addEventListener('input', () => isUserInteracting = true);
        
        progressBar.addEventListener('change', (event) => {
            const newPositionProcent = event.currentTarget.value;
            const newPositionMs = (durationMs / maxProgress) * newPositionProcent;

            player.seek(parseInt(newPositionMs));
            trackPosition.current = parseInt(newPositionProcent);
            isUserInteracting = false; // Desmarca a interação do usuário
        });
    });
    

    const playerController = useCallback((event, data) => {
        const playerEvents = {
            "Load": (track) => setTrack(track),
            "Playing": ({position_ms}) => {
                setPlayBtn(playOptions.pause);
                trackRefData.current.position = position_ms;
                clearInterval(intervalTrackBar.current);
                updateTrackPosition();
            },
            "Paused": () => {
                setPlayBtn(playOptions.play);
                clearInterval(intervalTrackBar.current);
            }
        }

        playerEvents[event](data);
    });

    useEffect(() => {
        listen('player://client_handler', (e) => {
            const { event, data } = e.payload;
            playerController(event, data);
        })
    }, []);

    useEffect(() => {trackRefData.current = track}, [track]);

    return (
        <div className='musicbar'>
            <div className='current-music'>
                <img src={track?.image} alt="" />
                <div className='data'>
                    <span className='title'>{track?.name}</span>
                    <span className='band'>{ track?.artist }</span>
                </div>
                <span style={{color: "white"}} id='heart'>&#x2764;</span>
            </div>
            <div className="controllers">
                <div className='options'>
                    <i className="material-icons">shuffle</i>
                    <i className="material-icons">skip_previous</i>
                    <i className="material-icons play_btn" onClick={() => playBtn.match(playOptions.play) 
                        ? player.play() 
                        : player.pause()}>{playBtn}</i>
                    <i className="material-icons">skip_next</i>
                    <i className="material-icons">all_inclusive</i>
                </div>
                <input type="range" id='track-bar-current' className='track-radio-controller' defaultValue={0} min={0} max={100} />
            </div>
            <div className='geral_conf'>
                <i className="material-icons">volume_up</i>
                <input type="range" className='volume_bar' defaultValue={100} min={0} max={100} onChange={(el) => player.setVolume(parseInt(el.target.value))}/>
            </div>
        </div>
    );
}

export default Musicbar;