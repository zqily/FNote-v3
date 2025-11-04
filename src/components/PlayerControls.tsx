import { Pause, Play, SkipBack, SkipForward } from 'lucide-react';
import { useTauriCommands } from '../hooks/useTauriCommands';
import { usePlaybackStatus } from '../hooks/useTauriEvents';
import { useAppStore } from '../store';

const formatTime = (seconds: number) => {
    const minutes = Math.floor(seconds / 60);
    const remainingSeconds = Math.floor(seconds % 60);
    return `${minutes}:${remainingSeconds.toString().padStart(2, '0')}`;
};

export function PlayerControls() {
    const status = usePlaybackStatus();
    const library = useAppStore((state) => state.library);
    const { pausePlayback, resumePlayback } = useTauriCommands();

    const currentTrack = library.find((t) => t.id === status?.current_track_id);
    const progress = status && status.duration_secs > 0 ? (status.elapsed_secs / status.duration_secs) * 100 : 0;

    if (!status || !currentTrack) {
        return (
            <footer className="h-24 bg-background border-t border-surface flex items-center justify-center text-secondary">
                No track playing
            </footer>
        );
    }

    return (
        <footer className="h-24 bg-surface p-4 flex flex-col justify-center gap-2">
            <div className="flex items-center gap-4">
                <div className="w-1/4">
                    <p className="font-bold truncate">{currentTrack.title}</p>
                    <p className="text-sm text-secondary truncate">{currentTrack.artist}</p>
                </div>

                <div className="flex-1 flex flex-col items-center gap-2">
                    {/* Playback Buttons */}
                    <div className="flex items-center gap-6">
                        <button className="text-secondary hover:text-primary transition-colors">
                            <SkipBack />
                        </button>
                        <button
                            onClick={status.is_playing ? pausePlayback : resumePlayback}
                            className="w-10 h-10 flex items-center justify-center rounded-full bg-primary text-background hover:bg-accent transition-colors"
                        >
                            {status.is_playing ? <Pause size={24} /> : <Play size={24} />}
                        </button>
                        <button className="text-secondary hover:text-primary transition-colors">
                            <SkipForward />
                        </button>
                    </div>
                </div>
                <div className="w-1/4" />
            </div>

            {/* Progress Bar */}
            <div className="flex items-center gap-2 text-xs text-secondary">
                <span>{formatTime(status.elapsed_secs)}</span>
                <div className="w-full bg-background rounded-full h-1">
                    <div
                        className="bg-accent h-1 rounded-full"
                        style={{ width: `${progress}%` }}
                    />
                </div>
                <span>{formatTime(status.duration_secs)}</span>
            </div>
        </footer>
    );
}