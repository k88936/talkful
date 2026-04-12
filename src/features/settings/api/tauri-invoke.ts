import {invoke} from '@tauri-apps/api/core';

type TauriInvoke = (command: string, args?: Record<string, unknown>) => Promise<unknown>;

interface TauriInternals {
    invoke?: TauriInvoke;
}

interface TauriRuntimeGlobal {
    __TAURI_INTERNALS__?: TauriInternals;
}

const TAURI_RUNTIME_UNAVAILABLE_MESSAGE =
    'Tauri runtime is unavailable. Settings API requires a Tauri webview. Start the app with `pnpm tauri dev` or run the packaged desktop app.';

const assertTauriRuntimeAvailable = (): void => {
    const runtime = globalThis as typeof globalThis & TauriRuntimeGlobal;
    if (typeof runtime.__TAURI_INTERNALS__?.invoke !== 'function') {
        throw new Error(TAURI_RUNTIME_UNAVAILABLE_MESSAGE);
    }
};

export const invokeTauriCommand = async <T>(
    command: string,
    args?: Record<string, unknown>,
): Promise<T> => {
    assertTauriRuntimeAvailable();
    return invoke<T>(command, args);
};
