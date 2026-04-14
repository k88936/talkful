import {FormEvent, useState} from 'react';

import Group from '@jetbrains/ring-ui-built/components/group/group';
import Island, {Content, Header} from '@jetbrains/ring-ui-built/components/island/island';
import Loader from '@jetbrains/ring-ui-built/components/loader/loader';
import Text from '@jetbrains/ring-ui-built/components/text/text';

import {relaunchApp} from '@/features/settings/api/process.ts';
import {ModelDownloadDialog} from '@/features/settings/components/ModelDownloadDialog.tsx';
import {SettingsForm} from '@/features/settings/components/SettingsForm.tsx';
import {useSettingsState} from '@/features/settings/hooks/useSettingsState.ts';
import {confirmRestartAfterSave} from '@/pages/settings/restart-confirm-dialog.ts';

export const SettingsPage = () => {
    const {settings, isLoading, isSaving, error, updateSettings, resetSettingsToDefaults, saveSettings} = useSettingsState();
    const [isModelDownloadDialogVisible, setModelDownloadDialogVisible] = useState(false);

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

    const handleSubmit = async (event: FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        await saveSettings();

        const shouldRestart = await confirmRestartAfterSave();
        if (shouldRestart) {
            await relaunchApp();
        }
    };

    return (
        <Island className="w-full max-w-3xl">
            <Header border>Settings</Header>
            <Content>
                <Group className="flex w-full flex-col gap-4">
                    <Text>
                        Configure autostart and hotkey persistence.
                    </Text>
                    <SettingsForm
                        settings={settings}
                        isSaving={isSaving}
                        onSettingsChange={updateSettings}
                        onResetDefaults={resetSettingsToDefaults}
                        onOpenModelDownloadDialog={() => setModelDownloadDialogVisible(true)}
                        onSubmit={handleSubmit}
                    />
                    <ModelDownloadDialog
                        show={isModelDownloadDialogVisible}
                        onClose={() => setModelDownloadDialogVisible(false)}
                    />
                </Group>
            </Content>
        </Island>
    );
};
