import {relaunch} from '@tauri-apps/plugin-process';

export const relaunchApp = async (): Promise<void> => {
    await relaunch();
};
