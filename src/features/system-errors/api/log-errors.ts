import {invoke} from '@tauri-apps/api/core';

const GET_LOG_ERRORS_COMMAND = 'get_log_errors';

export const getLogErrors = async (): Promise<string[]> => {
    return invoke<string[]>(GET_LOG_ERRORS_COMMAND);
};
