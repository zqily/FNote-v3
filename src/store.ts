import { create } from 'zustand';
import { Track } from './types';

interface AppState {
  library: Track[];
  setLibrary: (tracks: Track[]) => void;
}

export const useAppStore = create<AppState>((set) => ({
  library: [],
  setLibrary: (tracks) => set({ library: tracks }),
}));