import React, { useEffect, useRef, useState } from 'react';
import { useNavigate } from "react-router-dom";

import Musicbar from "../layouts/Musicbar/Musicbar";
import Navbar from '../layouts/Navbar/Navbar';
import Container from '../layouts/Container/Container';

import './home.css';

function Home() {

    const navegate = useNavigate();
    const [ currentPlaylist, setPlaylist ] = useState(null);
    const [ currentTrack, setTrack ] = useState(null);

    useEffect(() => {
        const accessToken = localStorage.getItem("accessToken");

        if(!accessToken) location.href = "/auth/login";
    }, []);



    return (
        <>
            <div className='home'>
                <Navbar setPlaylist={setPlaylist} />
                <Container currentPlaylist={currentPlaylist} setTrack={setTrack} />
            </div>
            <Musicbar />
        </>
    );
}

export default Home;