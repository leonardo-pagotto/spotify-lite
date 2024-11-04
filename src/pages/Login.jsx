import React, { useEffect, useRef } from 'react';
import { useNavigate } from "react-router-dom";

import { invoke } from '@tauri-apps/api';

import "./login.css";
import spotifyLogo from "/src/assets/icons/spotify.svg";

function Login() {

    const navegate = useNavigate();
    const loginForm = {
        email: useRef(null),
        password: useRef(null)
    };

    async function init(){
        let accessToken = localStorage.getItem('accessToken');
        
        if(accessToken) {
            try {
                await invoke("validate_access_token", { accessToken });
                navegate("/");
            } catch (err) {
                localStorage.removeItem("access_token");
            }

            return;
        };
    }

    async function Login(){
        const emailElement = loginForm.email.current;
        const passwordElement = loginForm.password.current;

        if(!emailElement.value || !passwordElement.value) return;

        invoke("login_session", {
            email: emailElement.value,
            password: passwordElement.value
        })
            .then((result) => {
                localStorage.setItem('accessToken', result.access_token);
                navegate("/");
            })
            .catch((err) => {
                console.error(err);
            })
        ;
    }

    useEffect(() => {
        init();
    }, []);

    return (
        <div className='login_page'>
            {/* <button onClick={() => {tokenManger.requestExplicit(location.href)}} className='spotify_login_btn'>
                LOGIN WITH SPOTIFY 
                <img src={spotifyLogo} alt="" />
            </button> */}
            <div className="formulary">
                <div className='header'>
                    <img src={spotifyLogo} className='img' alt="" />
                    <span className='title'>Spotify Lite</span>
                </div>
                <input type="text" className='input' placeholder='Email' required name='email' ref={loginForm.email} />
                <input type="password" className='input' placeholder='Password' ref={loginForm.password} />
                <label className='lembrese'> 
                    <span>Remember-me</span>
                    <input type="checkbox" />
                    <span className="checkmark"></span>
                </label>
                <input type="submit" value="LOGIN" className='spotify_login_btn login_btn' onClick={Login} />
            </div>
        </div>
    );
}

export default Login;