import {listen} from '@tauri-apps/api/event';
import {useCallback, useEffect, useRef, useState} from 'react';

import {ErrorBannerItem} from '@/features/system-errors/model/error-banner.ts';

interface UseErrorEventBannersResult {
    errors: ErrorBannerItem[];
    dismissError: (id: number) => void;
}

export const useErrorEventBanners = (): UseErrorEventBannersResult => {
    const [errors, setErrors] = useState<ErrorBannerItem[]>([]);
    const nextId = useRef(0);

    const dismissError = useCallback((id: number) => {
        setErrors(previous => previous.filter(error => error.id !== id));
    }, []);

    useEffect(() => {
        let isActive = true;
        let unlisten: (() => void) | null = null;

        void listen<string>('error', event => {
            const id = nextId.current;
            nextId.current += 1;
            setErrors(previous => [...previous, {id, message: event.payload}]);
        }).then(handler => {
            if (!isActive) {
                handler();
                return;
            }
            unlisten = handler;
        });

        return () => {
            isActive = false;
            if (unlisten !== null) {
                unlisten();
            }
        };
    }, []);

    return {
        errors,
        dismissError,
    };
};
