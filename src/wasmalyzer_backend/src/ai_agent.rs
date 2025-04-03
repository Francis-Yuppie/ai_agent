use quote::ToTokens;
use reqwest;
use serde_json;
use std::collections::HashMap;
use syn::{parse_file, File}; // Import the ToTokens trait.

#[ic_cdk::update]
pub async fn analyze_github_repo(repo_url: String) -> Result<String, String> {
    let repo_contents = get_repo_contents(repo_url).await?;

    let parsed_code = parse_code(repo_contents)?;

    let vulnerabilities = detect_vulnerabilities(parsed_code).await?;

    let code_quality = HashMap::new();

    let report = generate_report(vulnerabilities, code_quality)?;

    store_report(report.clone())?;

    Ok(report)
}

async fn get_repo_contents(repo_url: String) -> Result<HashMap<String, String>, String> {
    let repo_url = repo_url.trim_end_matches('/');

    // Extract owner and repo name from the URL
    let parts: Vec<&str> = repo_url.split('/').collect();
    if parts.len() < 2 {
        return Err("Invalid GitHub repository URL".to_string());
    }
    let owner = parts[parts.len() - 2];
    let repo = parts[parts.len() - 1];

    let url = format!("https://api.github.com/repos/{}/{}/contents", owner, repo);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "icp-code-analyzer") // Required by GitHub API
        .send()
        .await
        .map_err(|e| format!("Failed to fetch repository contents: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("GitHub API returned error: {}", response.status()));
    }

    let json: Vec<serde_json::Value> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse JSON response: {}", e))?;

    let mut contents = HashMap::new();
    for item in json {
        if item["type"].as_str() == Some("file") {
            let file_name = item["name"].as_str().unwrap_or("");
            let download_url = item["download_url"].as_str().unwrap_or("");

            let file_response = client
                .get(download_url)
                .send()
                .await
                .map_err(|e| format!("Failed to download file: {}", e))?;

            if file_response.status().is_success() {
                let file_content = file_response
                    .text()
                    .await
                    .map_err(|e| format!("Failed to read file content: {}", e))?;
                contents.insert(file_name.to_string(), file_content);
            }
        }
    }
    Ok(contents)
}

fn parse_code(repo_contents: HashMap<String, String>) -> Result<HashMap<String, File>, String> {
    let mut parsed_code = HashMap::new();
    for (file_name, file_content) in repo_contents {
        if file_name.ends_with(".rs") {
            match parse_file(&file_content) {
                Ok(file) => {
                    parsed_code.insert(file_name, file);
                }
                Err(e) => {
                    ic_cdk::println!("Failed to parse {}: {}", file_name, e);
                }
            }
        }
    }
    Ok(parsed_code)
}

async fn detect_vulnerabilities(parsed_code: HashMap<String, File>) -> Result<Vec<String>, String> {
    let mut vulnerabilities = Vec::new();
    let client = reqwest::Client::new();

    for (file_name, file) in parsed_code {
        let file_content = file.to_token_stream().to_string(); // Correct conversion

        let prompt = format!(
            "Analyze the following Rust code for security vulnerabilities:\n\n{}\n\nList any potential vulnerabilities and explain them.",
            file_content
        );

        let request_body = serde_json::json!({
            "model": "gpt-3.5-turbo",
            "messages": [{"role": "user", "content": prompt}]
        });

        let chatgpt_response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header(
                "Authorization",
                format!("Bearer {}", "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"),
            )
            .json(&request_body)
            .send()
            .await
            .map_err(|e| format!("Failed to call ChatGPT API: {}", e))?;

        if chatgpt_response.status().is_success() {
            let chatgpt_json: serde_json::Value = chatgpt_response
                .json()
                .await
                .map_err(|e| format!("Failed to parse ChatGPT response: {}", e))?;

            if let Some(content) = chatgpt_json["choices"][0]["message"]["content"].as_str() {
                vulnerabilities.push(format!("File: {}\n{}", file_name, content));
            }
        } else {
            return Err(format!(
                "ChatGPT API returned error: {:?}",
                chatgpt_response.status()
            ));
        }
    }
    Ok(vulnerabilities)
}

fn generate_report(
    vulnerabilities: Vec<String>,
    code_quality: HashMap<String, String>,
) -> Result<String, String> {
    let mut report = String::new();
    report.push_str("Vulnerability Report:\n");
    for v in vulnerabilities {
        report.push_str(&format!("{}\n\n", v));
    }

    report.push_str("\nCode Quality Report:\n");
    for (filename, quality) in code_quality {
        report.push_str(&format!("{}: {}\n", filename, quality));
    }

    Ok(report)
}

fn store_report(report: String) -> Result<(), String> {
    // Implement report storage logic here
    ic_cdk::println!("Report: {}", report); // Placeholder for now
    Ok(())
}
