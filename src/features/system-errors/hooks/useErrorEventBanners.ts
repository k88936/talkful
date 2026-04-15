import {listen} from '@tauri-apps/api/event';
import {useCallback, useEffect, useRef, useState} from 'react';

import {getLogErrors} from '@/features/system-errors/api/log-errors.ts';
import {ErrorBannerItem} from '@/features/system-errors/model/error-banner.ts';

interface UseErrorEventBannersResult {
    errors: ErrorBannerItem[];
    dismissError: (id: number) => void;
}

export const useErrorEventBanners = (): UseErrorEventBannersResult => {
    const [errors, setErrors] = useState<ErrorBannerItem[]>([]);
    const nextId = useRef(0);

    const appendError = useCallback((message: string) => {
        const id = nextId.current;
        nextId.current += 1;
        setErrors(previous => [...previous, {id, message}]);
    }, []);

    const dismissError = useCallback((id: number) => {
        setErrors(previous => previous.filter(error => error.id !== id));
    }, []);

    useEffect(() => {
        let isActive = true;
        let unlisten: (() => void) | null = null;

        void listen<string>('error', event => {
            appendError(event.payload);
        }).then(handler => {
            if (!isActive) {
                handler();
                return;
            }
            unlisten = handler;
        }).catch(reason => {
            if (!isActive) {
                return;
            }
            const message = reason instanceof Error ? reason.message : String(reason);
            appendError(`failed to listen for error events: ${message}`);
        });

        void getLogErrors().then(messages => {
            if (!isActive) {
                return;
            }
            messages.forEach(appendError);
        }).catch(reason => {
            if (!isActive) {
                return;
            }
            const message = reason instanceof Error ? reason.message : String(reason);
            appendError(`failed to read log errors: ${message}`);
        });

        return () => {
            isActive = false;
            if (unlisten !== null) {
                unlisten();
            }
        };
    }, [appendError]);

    return {
        errors,
        dismissError,
    };
};
