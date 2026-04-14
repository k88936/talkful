import Banner from '@jetbrains/ring-ui-built/components/banner/banner';
import Group from '@jetbrains/ring-ui-built/components/group/group';

import {ErrorBannerItem} from '@/features/system-errors/model/error-banner.ts';

interface ErrorBannersProps {
    errors: ErrorBannerItem[];
    onDismiss: (id: number) => void;
}

export const ErrorBanners = ({errors, onDismiss}: ErrorBannersProps) => {
    if (errors.length === 0) {
        return null;
    }

    return (
        <Group className="flex w-full flex-col gap-2 pb-4">
            {errors.map(error => (
                <Banner
                    key={error.id}
                    mode="error"
                    title="Error"
                    withIcon
                    onClose={() => onDismiss(error.id)}
                >
                    {error.message}
                </Banner>
            ))}
        </Group>
    );
};
