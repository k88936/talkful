import {disable, enable, isEnabled} from '@tauri-apps/plugin-autostart';

export const getAutostartEnabled = async (): Promise<boolean> => {
    return isEnabled();
};

export const setAutostartEnabled = async (enabled: boolean): Promise<void> => {
    if (enabled) {
        await enable();
        return;
    }
    await disable();
};
