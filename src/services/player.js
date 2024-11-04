import { invoke } from "@tauri-apps/api";

export async function playPauseSeter(state)
{
    if(state == "pause_circle_filled"){
        await pause();
        return;
    };

    await play();
}

export async function pause(){
    try {
        await invoke("pause_track");
    } catch (err) {
        console.log(err);
    }
}

export async function play(){
    try {
        await invoke("play_track");
    } catch (err) {
        console.log(err);
    }
}

export async function load(id){
    try {
        await invoke("load_track", {id});
    } catch (err) {
        console.log(err);
    }
}

export async function setVolume(value){
    try {
        await invoke("set_volume", {volume: value});
    } catch (err) {
        console.log(err);
    }

    return;
}

export async function seek(value){
    try {
        await invoke("set_track_position", {position: value});
    } catch (err) {
        console.log(err);
    }
}

export const playerEnventsTypes = {
    "Load": "Load",
    "Play": "Play",
    "Pause": "Pause",
    "Seek": "Seek",
    "End": "End",
}