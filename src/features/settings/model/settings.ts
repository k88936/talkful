export interface AppSettingsDto {
    autostart_enabled: boolean;
    hotkey_key: string;
}

export interface AppSettings {
    autostartEnabled: boolean;
    hotkeyKey: string;
}

export const fromAppSettingsDto = (dto: AppSettingsDto): AppSettings => {
    return {
        autostartEnabled: dto.autostart_enabled,
        hotkeyKey: dto.hotkey_key,
    };
};

export const toAppSettingsDto = (settings: AppSettings): AppSettingsDto => {
    return {
        autostart_enabled: settings.autostartEnabled,
        hotkey_key: settings.hotkeyKey,
    };
};
