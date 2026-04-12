import {invoke} from '@tauri-apps/api/core';
import {AppSettingsDto} from '@/features/settings/model/settings.ts';

const GET_SETTINGS_COMMAND = 'get_settings';
const SET_SETTINGS_COMMAND = 'set_settings';

export const getSettings = async (): Promise<AppSettingsDto> => {
    return invoke<AppSettingsDto>(GET_SETTINGS_COMMAND);
};

export const setSettings = async (newConfig: AppSettingsDto | null): Promise<AppSettingsDto> => {
    return invoke<AppSettingsDto>(SET_SETTINGS_COMMAND, {newConfig});
};
