import {useCallback, useEffect, useState} from 'react';

import {getSettings, setSettings} from '@/features/settings/api/settings-client.ts';
import {AppSettings, fromAppSettingsDto, toAppSettingsDto} from '@/features/settings/model/settings.ts';

interface UseSettingsStateResult {
    settings: AppSettings | null;
    isLoading: boolean;
    isSaving: boolean;
    error: Error | null;
    updateSettings: (next: AppSettings) => void;
    saveSettings: () => Promise<void>;
}

const toError = (reason: unknown): Error => {
    if (reason instanceof Error) {
        return reason;
    }
    return new Error(String(reason));
};

export const useSettingsState = (): UseSettingsStateResult => {
    const [settings, setSettingsState] = useState<AppSettings | null>(null);
    const [isSaving, setIsSaving] = useState(false);
    const [error, setError] = useState<Error | null>(null);

    useEffect(() => {
        let isMounted = true;

        void getSettings()
            .then(dto => {
                if (!isMounted) {
                    return;
                }
                setSettingsState(fromAppSettingsDto(dto));
            })
            .catch(reason => {
                if (!isMounted) {
                    return;
                }
                setError(toError(reason));
            });

        return () => {
            isMounted = false;
        };
    }, []);

    const updateSettings = useCallback((next: AppSettings) => {
        setSettingsState(next);
    }, []);

    const saveSettings = useCallback(async () => {
        if (settings === null) {
            throw new Error('settings state is not initialized');
        }

        setIsSaving(true);
        try {
            const persisted = await setSettings(toAppSettingsDto(settings));
            setSettingsState(fromAppSettingsDto(persisted));
        } catch (reason: unknown) {
            setError(toError(reason));
        } finally {
            setIsSaving(false);
        }
    }, [settings]);

    return {
        settings,
        isLoading: settings === null && error === null,
        isSaving,
        error,
        updateSettings,
        saveSettings,
    };
};
