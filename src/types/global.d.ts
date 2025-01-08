interface TauriCore {
    invoke: (cmd: string, args?: Record<string, unknown>) => Promise<any>;
    // Add other methods from __TAURI__.core as needed
}

interface Tauri {
    core: TauriCore;
    // Add other namespaces under __TAURI__ if needed
}

interface Window {
    __TAURI__: Tauri;
}
