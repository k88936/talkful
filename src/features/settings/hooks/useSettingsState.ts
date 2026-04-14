import {useCallback, useEffect, useState} from 'react';

import {getAutostartEnabled, setAutostartEnabled} from '@/features/settings/api/autostart.ts';
import {getSettings, setSettings} from '@/features/settings/api/settings.ts';
import {AppSettings, fromAppSettingsDto, toAppSettingsDto} from '@/features/settings/model/settings.ts';

interface UseSettingsStateResult {
    settings: AppSettings | null;
    isLoading: boolean;
    isSaving: boolean;
    error: Error | null;
    updateSettings: (next: AppSettings) => void;
    resetSettingsToDefaults: () => void;
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

        void Promise.all([getSettings(), getAutostartEnabled()])
            .then(([dto, autostartEnabled]) => {
                if (!isMounted) {
                    return;
                }
                setSettingsState(fromAppSettingsDto(dto, autostartEnabled));
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

    const resetSettingsToDefaults = useCallback(() => {
        setIsSaving(true);
        void (async () => {
            try {
                const [persisted] = await Promise.all([setSettings(null), setAutostartEnabled(false)]);
                const autostartEnabled = await getAutostartEnabled();
                setSettingsState(fromAppSettingsDto(persisted, autostartEnabled));
            } catch (reason: unknown) {
                setError(toError(reason));
            } finally {
                setIsSaving(false);
            }
        })();
    }, []);

    const saveSettings = useCallback(async () => {
        if (settings === null) {
            throw new Error('settings state is not initialized');
        }

        setIsSaving(true);
        try {
            const persisted = await setSettings(toAppSettingsDto(settings));
            await setAutostartEnabled(settings.autostartEnabled);
            const autostartEnabled = await getAutostartEnabled();
            setSettingsState(fromAppSettingsDto(persisted, autostartEnabled));
        } catch (reason: unknown) {
            const error = toError(reason);
            setError(error);
            throw error;
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
        resetSettingsToDefaults,
        saveSettings,
    };
};
