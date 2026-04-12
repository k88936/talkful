import confirm from '@jetbrains/ring-ui-built/components/confirm-service/confirm-service';

export const confirmRestartAfterSave = async (): Promise<boolean> => {
    return confirm({
        text: 'Settings were saved successfully.',
        description: 'Restart now to apply the changes?',
        confirmLabel: 'Restart now',
        rejectLabel: 'Later',
        cancelIsDefault: true,
    })
        .then(() => true)
        .catch((reason: unknown) => {
            if (reason === undefined) {
                return false;
            }
            throw reason;
        });
};
