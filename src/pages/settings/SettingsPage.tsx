import {FormEvent} from 'react';

import Group from '@jetbrains/ring-ui-built/components/group/group';
import Island, {Content, Header} from '@jetbrains/ring-ui-built/components/island/island';
import Loader from '@jetbrains/ring-ui-built/components/loader/loader';
import Text from '@jetbrains/ring-ui-built/components/text/text';

import {SettingsForm} from '@/features/settings/components/SettingsForm.tsx';
import {useSettingsState} from '@/features/settings/hooks/useSettingsState.ts';

export const SettingsPage = () => {
    const {settings, isLoading, isSaving, error, updateSettings, saveSettings} = useSettingsState();

    if (error !== null) {
        throw error;
    }

    if (isLoading || settings === null) {
        return (
            <Island className="w-full max-w-3xl">
                <Header border>Settings</Header>
                <Content>
                    <Group className="flex w-full justify-center py-8">
                        <Loader/>
                    </Group>
                </Content>
            </Island>
        );
    }

    const handleSubmit = (event: FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        void saveSettings();
    };

    return (
        <Island className="w-full max-w-3xl">
            <Header border>Settings</Header>
            <Content>
                <Group className="flex w-full flex-col gap-4">
                    <Text>
                        Configure model paths, startup mode, and hotkey persistence.
                    </Text>
                    <SettingsForm
                        settings={settings}
                        isSaving={isSaving}
                        onSettingsChange={updateSettings}
                        onSubmit={handleSubmit}
                    />
                </Group>
            </Content>
        </Island>
    );
};
