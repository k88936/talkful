import {invoke} from '@tauri-apps/api/core';

const GET_STARTUP_ERRORS_COMMAND = 'get_startup_errors';

export const getStartupErrors = async (): Promise<string[]> => {
    return invoke<string[]>(GET_STARTUP_ERRORS_COMMAND);
};
