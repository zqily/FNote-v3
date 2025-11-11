<script>
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import "../app.css";

  /** @type {'none' | 'playlist' | 'single'} */
  let loopMode = "none";

  // When the component first mounts, fetch the initial state from Rust.
  onMount(async () => {
    try {
      loopMode = await invoke("get_loop_mode");
    } catch (e) {
      console.error("Failed to get initial loop mode:", e);
    }
  });

  // Cycles through the loop modes and calls the Rust backend to update the state.
  async function cycleLoopMode() {
    let nextMode;
    if (loopMode === "none") {
      nextMode = "playlist";
    } else if (loopMode === "playlist") {
      nextMode = "single";
    } else {
      nextMode = "none";
    }
    
    try {
      await invoke("set_loop_mode", { mode: nextMode });
      loopMode = nextMode; // Update local state only on success
    } catch (e) {
      console.error("Failed to set loop mode:", e);
    }
  }
</script>

<!-- 
  Root container for the entire application.
  - h-screen, w-screen: Fills the entire viewport.
  - bg-zinc-950: Sets the main dark background color.
  - text-neutral-200: Sets a default light text color.
  - flex: Establishes a flexbox context for the main layout.
-->
<div class="h-screen w-screen bg-zinc-950 text-neutral-200 flex overflow-hidden">
  
  <!-- 
    Left Sidebar Panel
    - w-64: Sets a fixed width of 16rem (256px).
    - flex-shrink-0: Prevents the sidebar from shrinking when the window narrows.
    - bg-zinc-900: A slightly lighter shade for visual separation.
    - p-4: Adds some internal padding.
  -->
  <aside class="w-64 flex-shrink-0 bg-zinc-900 p-4">
    <!-- App Icon and Title Placeholder -->
    <div class="flex items-center space-x-2 mb-8">
      <img src="/fnote.png" alt="FNote Logo" class="h-8 w-8" />
      <h1 class="text-xl font-bold">FNote</h1>
    </div>
    <!-- Sidebar content placeholder -->
    <div class="h-48 bg-zinc-800 rounded-md animate-pulse"></div>
  </aside>

  <!-- 
    Main Content Area Wrapper
    - flex-1: Allows this area to grow and fill all remaining horizontal space.
    - flex, flex-col: Creates a vertical flexbox context to stack the content and player.
    - relative: Establishes a positioning context for potential future floating elements.
  -->
  <main class="flex-1 flex flex-col relative">
    
    <!-- 
      Central Content View (Album Art Area)
      - flex-1: This element grows to fill all available vertical space not used by the player.
      - bg-zinc-800: A placeholder background color.
    -->
    <div class="flex-1 bg-zinc-800/50 m-2 rounded-lg">
      <!-- Placeholder for page content -->
      <slot />
    </div>

    <!-- 
      Bottom Player Control Bar
      - Updated to contain a functional loop mode button.
    -->
    <div class="h-24 flex-shrink-0 bg-black/30 m-2 mb-4 rounded-lg flex items-center justify-center p-4">
      <!-- Player controls will go here -->
      <div class="flex items-center space-x-4">
        <p class="text-sm text-zinc-400">Player Controls Placeholder</p>
        <button 
          on:click={cycleLoopMode}
          class="p-2 rounded-full hover:bg-zinc-700 transition-colors focus:outline-none focus:ring-2 focus:ring-emerald-500"
          title="Change Loop Mode (Current: {loopMode})"
        >
          {#if loopMode === 'playlist'}
            <!-- Loop Playlist Icon -->
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-emerald-500"><path d="m17 2 4 4-4 4"/><path d="M3 11v-1a4 4 0 0 1 4-4h14"/><path d="m7 22-4-4 4-4"/><path d="M21 13v1a4 4 0 0 1-4 4H3"/></svg>
          {:else if loopMode === 'single'}
            <!-- Loop Single Icon -->
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-emerald-500"><path d="m17 2 4 4-4 4"/><path d="M3 11v-1a4 4 0 0 1 4-4h14"/><path d="m7 22-4-4 4-4"/><path d="M21 13v1a4 4 0 0 1-4 4H3"/><path d="M11 10h1v4"/></svg>
          {:else}
            <!-- No Loop Icon (Slightly transparent) -->
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-zinc-400"><path d="m17 2 4 4-4 4"/><path d="M3 11v-1a4 4 0 0 1 4-4h14"/><path d="m7 22-4-4 4-4"/><path d="M21 13v1a4 4 0 0 1-4 4H3"/></svg>
          {/if}
        </button>
      </div>
    </div>
  </main>

</div>