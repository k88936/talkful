import {FormEvent} from 'react';

import Button from '@jetbrains/ring-ui-built/components/button/button';
import Group from '@jetbrains/ring-ui-built/components/group/group';
import Panel from '@jetbrains/ring-ui-built/components/panel/panel';
import Text from '@jetbrains/ring-ui-built/components/text/text';
import Toggle from '@jetbrains/ring-ui-built/components/toggle/toggle';

import {AppSettings} from '@/features/settings/model/settings.ts';
import {SettingTextInput} from '@/features/settings/components/SettingTextInput.tsx';

interface SettingsFormProps {
    settings: AppSettings;
    isSaving: boolean;
    onSettingsChange: (next: AppSettings) => void;
    onResetDefaults: () => void;
    onOpenModelDownloadDialog: () => void;
    onSubmit: (event: FormEvent<HTMLFormElement>) => void;
}

export const SettingsForm = ({
    settings,
    isSaving,
    onSettingsChange,
    onResetDefaults,
    onOpenModelDownloadDialog,
    onSubmit,
}: SettingsFormProps) => {
    const updateField = <K extends keyof AppSettings>(field: K, value: AppSettings[K]) => {
        onSettingsChange({
            ...settings,
            [field]: value,
        });
    };

    return (
        <form className="w-full" onSubmit={onSubmit}>
            <Group className="flex w-full flex-col gap-4">
                <SettingTextInput
                    label="Hotkey key"
                    value={settings.hotkeyKey}
                    help="Persisted now, active after restart"
                    onChange={value => updateField('hotkeyKey', value)}
                />
                <Button type="button" onClick={onOpenModelDownloadDialog} disabled={isSaving}>
                    Download ASR model files
                </Button>
                <Toggle
                    checked={settings.autostartEnabled}
                    onChange={event => updateField('autostartEnabled', event.currentTarget.checked)}
                >
                    <Text>
                        Enable autostart on system launch
                    </Text>
                </Toggle>
                <Panel>
                    <Button type="button" onClick={onResetDefaults} disabled={isSaving}>
                        Restore defaults
                    </Button>
                    <Button primary type="submit" loader={isSaving} disabled={isSaving}>
                        Save settings
                    </Button>
                </Panel>
                <Text size={Text.Size.S} info>
                    Hotkey changes apply after app restart. Autostart is applied when you save.
                </Text>
            </Group>
        </form>
    );
};
