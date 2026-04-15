import {useEffect, useMemo, useState} from 'react';

import Button from '@jetbrains/ring-ui-built/components/button/button';
import Dialog from '@jetbrains/ring-ui-built/components/dialog/dialog';
import DropdownMenu from '@jetbrains/ring-ui-built/components/dropdown-menu/dropdown-menu';
import Group from '@jetbrains/ring-ui-built/components/group/group';
import Input, {Size} from '@jetbrains/ring-ui-built/components/input/input';
import Island, {Content, Header} from '@jetbrains/ring-ui-built/components/island/island';
import List from '@jetbrains/ring-ui-built/components/list/list';
import {type ListDataItem} from '@jetbrains/ring-ui-built/components/list/consts';
import Panel from '@jetbrains/ring-ui-built/components/panel/panel';
import PopupMenu from '@jetbrains/ring-ui-built/components/popup-menu/popup-menu';
import Text from '@jetbrains/ring-ui-built/components/text/text';

import {downloadModelFiles} from '@/features/settings/api/model-download.ts';
import {
    DEFAULT_MODEL_DOWNLOAD_SOURCE_KEY,
    MODEL_DOWNLOAD_SOURCE_OPTIONS,
    type ModelDownloadSourceOption,
    toModelDownloadSourceKey,
} from '@/features/settings/model/model-download.ts';

interface ModelDownloadDialogProps {
    show: boolean;
    onClose: () => void;
}

const toErrorMessage = (reason: unknown): string => {
    if (reason instanceof Error) {
        return reason.message;
    }
    return String(reason);
};

export const ModelDownloadDialog = ({show, onClose}: ModelDownloadDialogProps) => {
    const [selectedSourceKey, setSelectedSourceKey] = useState(DEFAULT_MODEL_DOWNLOAD_SOURCE_KEY);
    const [httpProxyUrl, setHttpProxyUrl] = useState('');
    const [isDownloading, setIsDownloading] = useState(false);
    const [downloadError, setDownloadError] = useState<string | null>(null);
    const [isSuccess, setIsSuccess] = useState(false);

    useEffect(() => {
        if (show) {
            return;
        }
        setSelectedSourceKey(DEFAULT_MODEL_DOWNLOAD_SOURCE_KEY);
        setHttpProxyUrl('');
        setIsDownloading(false);
        setDownloadError(null);
        setIsSuccess(false);
    }, [show]);

    const sourceOptionsByModel = useMemo(() => {
        const grouped = new Map<string, ModelDownloadSourceOption[]>();
        MODEL_DOWNLOAD_SOURCE_OPTIONS.forEach(option => {
            const existing = grouped.get(option.model);
            if (existing === undefined) {
                grouped.set(option.model, [option]);
                return;
            }
            existing.push(option);
        });
        return Array.from(grouped.entries()).map(([model, options]) => ({
            model,
            options,
        }));
    }, []);

    const selectedSourceOption = useMemo(() => {
        const matched = MODEL_DOWNLOAD_SOURCE_OPTIONS.find(option => toModelDownloadSourceKey(option) === selectedSourceKey);
        if (matched === undefined) {
            throw new Error(`Unknown model download source key: ${selectedSourceKey}`);
        }
        return matched;
    }, [selectedSourceKey]);

    const selectedSourceLabel = `${selectedSourceOption.model}: ${selectedSourceOption.source}`;

    const filesListData = useMemo<ListDataItem[]>(() => {
        const listData: ListDataItem[] = [
            {
                key: 'files-group-title',
                rgItemType: List.ListProps.Type.TITLE,
                label: `Model: ${selectedSourceOption.model}`,
                description: `Source: ${selectedSourceOption.source}`,
            },
        ];

        selectedSourceOption.files.forEach((file, index) => {
            if (index > 0) {
                listData.push({
                    key: `file-separator-${index}`,
                    rgItemType: List.ListProps.Type.SEPARATOR,
                });
            }

            listData.push({
                key: `file-item-${index}-${file.local_file_name}`,
                rgItemType: List.ListProps.Type.ITEM,
                label: file.local_file_name,
                description: file.description,
                details: file.url,
            });
        });

        return listData;
    }, [selectedSourceOption]);

    const handleDownload = async (): Promise<void> => {
        setIsDownloading(true);
        setDownloadError(null);
        setIsSuccess(false);
        try {
            await downloadModelFiles({
                http_proxy_url: httpProxyUrl,
                files: selectedSourceOption.files,
            });
            setIsSuccess(true);
        } catch (reason: unknown) {
            setDownloadError(toErrorMessage(reason));
        } finally {
            setIsDownloading(false);
        }
    };

    const selectSourceFromMenuItem = (item: {key?: unknown} | null | undefined, closeMenu: (() => void) | undefined): void => {
        const sourceKey = item?.key;
        if (typeof sourceKey !== 'string') {
            throw new Error('Download source option key is missing.');
        }
        setSelectedSourceKey(sourceKey);
        closeMenu?.();
    };

    const handleCloseAttempt = (): void => {
        if (isDownloading) {
            return;
        }
        onClose();
    };

    return (
        <Dialog
            label="Model download"
            show={show}
            onCloseAttempt={handleCloseAttempt}
            trapFocus
            className="flex-1"
            autoFocusFirst
            showCloseButton
        >
            <Island className="max-w-full">
                <Header border>Download ASR model files</Header>
                <Content>
                    <Group className="flex w-full flex-col gap-3">
                        <Text>
                            Download source:
                        </Text>
                        <DropdownMenu anchor={selectedSourceLabel} menuProps={{closeOnSelect: false}}>
                            {props => (
                                <PopupMenu
                                    {...props}
                                    data={sourceOptionsByModel.map(group => ({
                                        key: group.model,
                                        rgItemType: List.ListProps.Type.CUSTOM,
                                        template: (
                                            <DropdownMenu
                                                anchor={group.model}
                                                data={group.options.map(option => ({
                                                    key: toModelDownloadSourceKey(option),
                                                    label: option.source,
                                                }))}
                                                onSelect={item =>
                                                    selectSourceFromMenuItem(
                                                        item as {key?: unknown} | null,
                                                        props.onCloseAttempt,
                                                    )
                                                }
                                            />
                                        ),
                                    }))}
                                />
                            )}
                        </DropdownMenu>
                        <Text>
                            Files to download:
                        </Text>
                        <List
                            data={filesListData}
                            onResize={() => {}}
                            renderOptimization={false}
                            shortcuts
                        />
                        <Input
                            label="HTTP proxy URL (optional)"
                            value={httpProxyUrl}
                            size={Size.L}
                            help="Use protocol://(user:pass@)host:port, Leave empty to honor proxy env or direct download"
                            onChange={event => setHttpProxyUrl(event.currentTarget.value)}
                        />
                        {downloadError === null ? null : (
                            <Text>
                                Download failed: {downloadError}
                            </Text>
                        )}
                        {isSuccess ? (
                            <Text>
                                Model files downloaded successfully.
                            </Text>
                        ) : null}
                    </Group>
                </Content>
                <Panel>
                    <Button primary loader={isDownloading} disabled={isDownloading} onClick={() => void handleDownload()}>
                        Download files
                    </Button>
                    <Button disabled={isDownloading} onClick={handleCloseAttempt}>
                        Close
                    </Button>
                </Panel>
            </Island>
        </Dialog>
    );
};
