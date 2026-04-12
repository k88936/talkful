import {FormEvent} from 'react';

import Button from '@jetbrains/ring-ui-built/components/button/button';
import Group from '@jetbrains/ring-ui-built/components/group/group';
import Text from '@jetbrains/ring-ui-built/components/text/text';
import Toggle from '@jetbrains/ring-ui-built/components/toggle/toggle';

import {AppSettings} from '@/features/settings/model/settings.ts';
import {SettingTextInput} from '@/features/settings/components/SettingTextInput.tsx';

interface SettingsFormProps {
    settings: AppSettings;
    isSaving: boolean;
    onSettingsChange: (next: AppSettings) => void;
    onSubmit: (event: FormEvent<HTMLFormElement>) => void;
}

export const SettingsForm = ({settings, isSaving, onSettingsChange, onSubmit}: SettingsFormProps) => {
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
                    label="ASR model filename"
                    value={settings.asrModelFilename}
                    help="Model file under ~/.talkful"
                    onChange={value => updateField('asrModelFilename', value)}
                />
                <SettingTextInput
                    label="ASR token filename"
                    value={settings.asrTokenFilename}
                    help="Token file under ~/.talkful"
                    onChange={value => updateField('asrTokenFilename', value)}
                />
                <SettingTextInput
                    label="Hotkey key"
                    value={settings.hotkeyKey}
                    help="Persisted now, active after restart"
                    onChange={value => updateField('hotkeyKey', value)}
                />
                <Toggle
                    checked={settings.autostartEnabled}
                    onChange={event => updateField('autostartEnabled', event.currentTarget.checked)}
                >
                    <Text>
                        Enable autostart on app launch
                    </Text>
                </Toggle>
                <Button primary type="submit" loader={isSaving} disabled={isSaving}>
                    Save settings
                </Button>
                <Text size={Text.Size.S} info>
                    Changes are saved immediately and applied after app restart.
                </Text>
            </Group>
        </form>
    );
};
