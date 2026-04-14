export interface AppSettingsDto {
    hotkey_key: string;
}

export interface AppSettings {
    autostartEnabled: boolean;
    hotkeyKey: string;
}

export const fromAppSettingsDto = (dto: AppSettingsDto, autostartEnabled: boolean): AppSettings => {
    return {
        autostartEnabled,
        hotkeyKey: dto.hotkey_key,
    };
};

export const toAppSettingsDto = (settings: AppSettings): AppSettingsDto => {
    return {
        hotkey_key: settings.hotkeyKey,
    };
};
