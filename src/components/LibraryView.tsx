import { Music2 } from 'lucide-react';
import { useTauriCommands } from '../hooks/useTauriCommands';
import { usePlaybackStatus } from '../hooks/useTauriEvents';
import { useAppStore } from '../store';

const formatDuration = (seconds: number) => {
    const minutes = Math.floor(seconds / 60);
    const remainingSeconds = seconds % 60;
    return `${minutes}:${remainingSeconds.toString().padStart(2, '0')}`;
};

export function LibraryView() {
    const library = useAppStore((state) => state.library);
    const { playTrack } = useTauriCommands();
    const playbackStatus = usePlaybackStatus();

    if (library.length === 0) {
        return (
            <div className="flex-1 flex flex-col items-center justify-center text-secondary">
                <Music2 size={48} className="mb-4" />
                <h2 className="text-xl">Your library is empty</h2>
                <p>Click "Add Music Folder" in the sidebar to get started.</p>
            </div>
        );
    }

    return (
        <div className="flex-1 overflow-y-auto p-4">
            <ul>
                {library.map((track) => {
                    const isPlaying = playbackStatus?.current_track_id === track.id;
                    return (
                        <li
                            key={track.id}
                            onClick={() => playTrack(track.id)}
                            className={`flex items-center justify-between p-3 rounded-md cursor-pointer hover:bg-surface ${isPlaying ? 'bg-surface text-accent' : ''}`}
                        >
                            <div className="flex flex-col">
                                <span className="font-medium">{track.title}</span>
                                <span className={`text-sm ${isPlaying ? 'text-accent' : 'text-secondary'}`}>
                                    {track.artist}
                                </span>
                            </div>
                            <span className="text-sm text-secondary">
                                {formatDuration(track.duration_secs)}
                            </span>
                        </li>
                    );
                })}
            </ul>
        </div>
    );
}