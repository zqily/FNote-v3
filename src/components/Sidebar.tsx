import { FolderPlus } from 'lucide-react';
import { useTauriCommands } from '../hooks/useTauriCommands';

export function Sidebar() {
    const { selectAndScanMusicDirectory } = useTauriCommands();

    return (
        <aside className="w-64 bg-background p-4 flex flex-col border-r border-surface">
            <div className="flex items-center mb-8">
                <h1 className="text-2xl font-bold text-accent">FNote</h1>
            </div>
            <nav>
                <button
                    onClick={selectAndScanMusicDirectory}
                    className="w-full flex items-center gap-3 px-4 py-2 rounded-md bg-surface text-primary hover:bg-accent hover:text-background transition-colors"
                >
                    <FolderPlus size={20} />
                    <span>Add Music Folder</span>
                </button>
            </nav>
        </aside>
    );
}