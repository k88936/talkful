import {AppSettingsDto} from '@/features/settings/model/settings.ts';
import {invokeTauriCommand} from '@/features/settings/api/tauri-invoke.ts';

const GET_SETTINGS_COMMAND = 'get_settings';
const SET_SETTINGS_COMMAND = 'set_settings';

export const getSettings = async (): Promise<AppSettingsDto> => {
    return invokeTauriCommand<AppSettingsDto>(GET_SETTINGS_COMMAND);
};

export const setSettings = async (newConfig: AppSettingsDto): Promise<AppSettingsDto> => {
    return invokeTauriCommand<AppSettingsDto>(SET_SETTINGS_COMMAND, {newConfig});
};
