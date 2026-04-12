import {invoke} from '@tauri-apps/api/core';

import {ModelDownloadRequest} from '@/features/settings/model/model-download.ts';

const DOWNLOAD_MODEL_FILES_COMMAND = 'download_model_files';

export const downloadModelFiles = async (request: ModelDownloadRequest): Promise<void> => {
    await invoke<void>(DOWNLOAD_MODEL_FILES_COMMAND, {request});
};
