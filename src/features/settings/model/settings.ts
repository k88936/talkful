export interface AppSettingsDto {
    asr_model_filename: string;
    asr_token_filename: string;
    autostart_enabled: boolean;
    hotkey_key: string;
}

export interface AppSettings {
    asrModelFilename: string;
    asrTokenFilename: string;
    autostartEnabled: boolean;
    hotkeyKey: string;
}

export const fromAppSettingsDto = (dto: AppSettingsDto): AppSettings => {
    return {
        asrModelFilename: dto.asr_model_filename,
        asrTokenFilename: dto.asr_token_filename,
        autostartEnabled: dto.autostart_enabled,
        hotkeyKey: dto.hotkey_key,
    };
};

export const toAppSettingsDto = (settings: AppSettings): AppSettingsDto => {
    return {
        asr_model_filename: settings.asrModelFilename,
        asr_token_filename: settings.asrTokenFilename,
        autostart_enabled: settings.autostartEnabled,
        hotkey_key: settings.hotkeyKey,
    };
};
