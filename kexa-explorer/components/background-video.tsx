'use client';

import { useState } from 'react';

const defaultVideo = 'https://pixabay.com/videos/download/video-5200_medium.mp4';

export function BackgroundVideo() {
  const [errored, setErrored] = useState(false);
  const source = process.env.NEXT_PUBLIC_BG_VIDEO_URL || defaultVideo;

  return (
    <div aria-hidden className="pointer-events-none fixed inset-0 -z-20 overflow-hidden">
      {!errored ? (
        <video
          className="h-full w-full object-cover opacity-70"
          autoPlay
          loop
          muted
          playsInline
          preload="metadata"
          onError={() => setErrored(true)}
        >
          <source src={source} type="video/mp4" />
        </video>
      ) : (
        <div className="h-full w-full animate-pulse bg-[radial-gradient(circle_at_15%_20%,#312e81,transparent_45%),radial-gradient(circle_at_80%_20%,#0e7490,transparent_35%),#050816]" />
      )}
      <div className="absolute inset-0 bg-gradient-to-b from-[#020617]/70 via-[#04091d]/75 to-[#020617]/90" />
      <div className="absolute inset-0 backdrop-blur-[2px]" />
      <div className="absolute inset-0 bg-[radial-gradient(circle_at_center,transparent_0%,rgba(2,6,23,0.58)_70%,rgba(2,6,23,0.88)_100%)]" />
      <div className="noise-layer absolute inset-0" />
    </div>
  );
}
