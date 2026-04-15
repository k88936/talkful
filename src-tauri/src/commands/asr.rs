use serde::Deserialize;
use std::path::Path;
use talkful_lib::shared::get_base_path;

#[derive(Debug, Deserialize)]
pub struct DownloadModelFileRequest {
    pub url: String,
    pub local_file_name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct DownloadModelFilesRequest {
    pub http_proxy_url: String,
    pub files: Vec<DownloadModelFileRequest>,
}

#[tauri::command]
pub fn get_model_directory_path() -> Result<String, String> {
    let model_directory_path = talkful_lib::shared::get_base_path().join("models");
    model_directory_path
        .into_os_string()
        .into_string()
        .map_err(|_| "model directory path is not valid UTF-8".to_string())
}

fn validate_local_file_name(local_file_name: &str, description: &str) -> Result<String, String> {
    let normalized_file_name = local_file_name.trim();
    if normalized_file_name.is_empty() {
        return Err(format!("local_file_name is empty for {}", description));
    }

    let path = Path::new(normalized_file_name);
    let component_count = path.components().count();
    if component_count != 1 {
        return Err(format!(
            "local_file_name must be a plain file name for {}",
            description
        ));
    }

    Ok(normalized_file_name.to_string())
}

#[tauri::command]
pub async fn download_model_files(request: DownloadModelFilesRequest) -> Result<(), String> {
    if request.files.is_empty() {
        return Err("files must not be empty".to_string());
    }

    let normalized_http_proxy_url = request.http_proxy_url.trim().to_string();

    let mut client_builder = reqwest::Client::builder();
    if !normalized_http_proxy_url.is_empty() {
        let proxy = reqwest::Proxy::all(&normalized_http_proxy_url)
            .map_err(|error| format!("invalid http_proxy_url: {error}"))?;
        client_builder = client_builder.proxy(proxy);
    }
    let client = client_builder
        .build()
        .map_err(|error| format!("failed to build HTTP client: {error}"))?;

    let model_directory_path = get_base_path().join("models");
    std::fs::create_dir_all(&model_directory_path)
        .map_err(|error| format!("failed to create model directory: {error}"))?;

    for file in request.files {
        let normalized_file_name = validate_local_file_name(&file.local_file_name, &file.description)?;
        let response = client
            .get(&file.url)
            .send()
            .await
            .map_err(|error| format!("failed to download {}: {error}", file.description))?;

        if !response.status().is_success() {
            return Err(format!(
                "failed to download {}: HTTP {}",
                file.description,
                response.status()
            ));
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|error| format!("failed to read response body for {}: {error}", file.description))?;

        let target_file_path = model_directory_path.join(normalized_file_name);
        std::fs::write(&target_file_path, bytes)
            .map_err(|error| format!("failed to write file {}: {error}", target_file_path.display()))?;
    }

    Ok(())
}
