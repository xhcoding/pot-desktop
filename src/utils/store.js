import { LazyStore } from '@tauri-apps/plugin-store';
import { appConfigDir, join } from '@tauri-apps/api/path';
import { invoke } from '@tauri-apps/api/core';
import { watch } from '@tauri-apps/plugin-fs';

export let store = new LazyStore();

export async function initStore() {
    const appConfigDirPath = await appConfigDir();
    const appConfigPath = await join(appConfigDirPath, 'config.json');
    store = new LazyStore(appConfigPath);
    // const unwatch = await watch(appConfigPath, async () => {
    //     await store.load();
    //     await invoke('reload_store');
    // }, { recursive: false });
}
