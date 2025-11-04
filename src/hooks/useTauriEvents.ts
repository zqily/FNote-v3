// src/hooks/useTauriEvents.ts

import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';
import { PlaybackStatus } from '../types';

// This hook is responsible for listening to a specific event from the Rust backend.
// It handles setting up the listener when the component mounts and cleaning it up
// when the component unmounts. This is crucial for preventing memory leaks.
export const usePlaybackStatus = () => {
    const [status, setStatus] = useState<PlaybackStatus | null>(null);

    useEffect(() => {
        // `listen` returns a promise that resolves to an `unlisten` function.
        const unlistenPromise = listen<PlaybackStatus>('playback_status_changed', (event) => {
            setStatus(event.payload);
        });

        // The cleanup function for the `useEffect` hook.
        // It will be called when the component that uses this hook is unmounted.
        return () => {
            unlistenPromise.then(unlistenFn => unlistenFn());
        };
    }, []); // The empty dependency array means this effect runs only once on mount.

    return status;
};