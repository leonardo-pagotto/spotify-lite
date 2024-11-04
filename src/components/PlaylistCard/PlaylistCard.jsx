import React from 'react'

function PlaylistCard({name, owner, image}) {
    return (
        <>
            <img src={image} alt="" />
            <div className='specifications'>
                <span className='name'>{name}</span>
                <span className='type'>Playlist • {owner}</span>
            </div>
        </>
    );
}

export default PlaylistCard;