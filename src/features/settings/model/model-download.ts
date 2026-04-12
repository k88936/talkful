export interface ModelDownloadFile {
    url: string;
    local_file_name: string;
    description: string;
}

export interface ModelDownloadSourceOption {
    model: string;
    source: string;
    files: readonly ModelDownloadFile[];
}

export const MODEL_DOWNLOAD_SOURCE_OPTIONS: readonly ModelDownloadSourceOption[] = [
    {
        model: 'paraformer-offline',
        source: 'github release',
        files: [
            {
                url: 'https://github.com/k88936/talkful/releases/download/model-paraformer-offline/paraformer-offline.model.int8.onnx',
                local_file_name: 'paraformer-offline.model.int8.onnx',
                description: 'paraformer model',
            },
            {
                url: 'https://github.com/k88936/talkful/releases/download/model-paraformer-offline/paraformer-offline.tokens.txt',
                local_file_name: 'paraformer-offline.tokens.txt',
                description: 'paraformer token file',
            },
        ],
    },
    {
        model: 'paraformer-offline',
        source: 'fucode release',
        files: [
            {
                url: 'https://gitcode.com/k88936/talkful/releases/download/model-paraformer-offline/paraformer-offline.model.int8.onnx',
                local_file_name: 'paraformer-offline.model.int8.onnx',
                description: 'ASR model',
            },
            {
                url: 'https://gitcode.com/k88936/talkful/releases/download/model-paraformer-offline/paraformer-offline.tokens.txt',
                local_file_name: 'paraformer-offline.tokens.txt',
                description: 'paraformer token file',
            },
        ],
    },
];

export const toModelDownloadSourceKey = (option: ModelDownloadSourceOption): string => {
    return `${option.model}::${option.source}`;
};

const getDefaultSourceKey = (options: readonly ModelDownloadSourceOption[]): string => {
    const firstOption = options[0];
    if (firstOption === undefined) {
        throw new Error('No model download source options configured.');
    }
    return toModelDownloadSourceKey(firstOption);
};

export const DEFAULT_MODEL_DOWNLOAD_SOURCE_KEY = getDefaultSourceKey(MODEL_DOWNLOAD_SOURCE_OPTIONS);

export interface ModelDownloadRequest {
    http_proxy_url: string;
    files: readonly ModelDownloadFile[];
}
