import { LibraryView } from './components/LibraryView';
import { PlayerControls } from './components/PlayerControls';
import { Sidebar } from './components/Sidebar';

function App() {
  return (
    <div className="h-screen w-screen flex flex-col bg-background text-primary antialiased">
      <div className="flex flex-1 overflow-hidden">
        <Sidebar />
        <main className="flex-1 flex flex-col">
          <LibraryView />
        </main>
      </div>
      <PlayerControls />
    </div>
  );
}
export default App;