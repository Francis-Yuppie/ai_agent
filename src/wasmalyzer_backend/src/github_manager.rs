use base64::decode;
use candid::CandidType;
// use candid::Nat;
use ic_cdk::api::management_canister::http_request::{
    http_request,
    CanisterHttpRequestArgument,
    HttpHeader,
    HttpMethod,
    HttpResponse,
    // TransformContext,
};
use ic_cdk_macros::query;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
// use std::convert::TryInto;

// Storage for user -> repo mapping
thread_local! {
    static USER_REPOS: std::cell::RefCell<HashMap<String, String>> = std::cell::RefCell::new(HashMap::new());
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct RepoAnalysisResult {
    vulnerabilities: Vec<Vulnerability>,
    code_quality: CodeQualityMetrics,
    suggestions: Vec<String>,
    code: String,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct Vulnerability {
    severity: String,
    description: String,
    location: String,
    suggestion: String,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct CodeQualityMetrics {
    complexity: f32,
    test_coverage: Option<f32>,
    style_violations: u32,
}

#[ic_cdk::update]
pub fn link_repository(repo_url: String) -> Result<(), String> {
    let user_id = ic_cdk::caller().to_text();

    if !repo_url.starts_with("https://github.com/") {
        return Err("Invalid GitHub repository URL".to_string());
    }

    USER_REPOS.with(|repos| {
        repos.borrow_mut().insert(user_id, repo_url);
    });

    Ok(())
}

#[ic_cdk::update]
pub async fn analyze_repository(repo_url: String) -> Result<RepoAnalysisResult, String> {
    let contents = fetch_repository_contents(&repo_url).await?;

    let analysis = perform_code_analysis(&contents)?;

    Ok(analysis)
}

async fn fetch_repository_contents(repo_url: &str) -> Result<String, String> {
    let api_path = repo_url.replace("https://github.com/", "");
    let url = format!("https://api.github.com/repos/{}/contents", api_path);

    // Use a personal access token for private repositories if needed
    let headers = vec![
        HttpHeader {
            name: "User-Agent".to_string(),
            value: "ICP-Code-Analyzer".to_string(),
        },
        HttpHeader {
            name: "Accept".to_string(),
            value: "application/vnd.github.v3+json".to_string(),
        },
        // Uncomment and replace with your personal access token if needed
        // HttpHeader {
        //     name: "Authorization".to_string(),
        //     value: "token YOUR_PERSONAL_ACCESS_TOKEN".to_string(),
        // },
    ];

    let request = CanisterHttpRequestArgument {
        url,
        max_response_bytes: Some(500_000),
        method: HttpMethod::GET,
        headers,
        body: None,
        transform: None,
    };

    let cycles: u128 = 100_000_000_000;

    let (response,): (HttpResponse,) = http_request(request, cycles)
        .await
        .map_err(|e| format!("HTTP request failed: {:?}", e))?;

    let content = String::from_utf8(response.body).map_err(|e| e.to_string())?;

    // Convert `response.status` to a valid status code (u64)
    let status_code = response.status.to_string().parse::<u64>().unwrap_or(0);

    if status_code != 200 {
        return Err(format!(
            "GitHub API request failed with status code {}: {}",
            status_code, content
        ));
    }

    // Parse the contents of the repository response (list of files)
    let files: Result<Vec<Value>, _> = serde_json::from_str(&content);

    match files {
        Ok(file_list) => {
            let mut full_code = String::new();

            for file in file_list {
                if let Some(path) = file.get("path").and_then(|v| v.as_str()) {
                    if let Some(base64_content) = file.get("content").and_then(|v| v.as_str()) {
                        // Decode the base64 content to actual code
                        let decoded_content = decode(base64_content)
                            .map_err(|e| format!("Failed to decode base64: {}", e))?;
                        if let Ok(code) = String::from_utf8(decoded_content) {
                            full_code.push_str(&format!("// File: {}\n{}\n\n", path, code));
                        }
                    }
                }
            }

            Ok(full_code)
        }
        Err(e) => {
            // If the response is not a valid list of files, log and return an error
            Err(format!("Failed to parse GitHub response: {}", e))
        }
    }
}

fn perform_code_analysis(contents: &str) -> Result<RepoAnalysisResult, String> {
    let mut vulnerabilities = Vec::new();
    let suggestions = Vec::new();

    if contents.contains("unsafe") {
        vulnerabilities.push(Vulnerability {
            severity: "Medium".to_string(),
            description: "Unsafe block detected".to_string(),
            location: "File.rs:42".to_string(),
            suggestion: "Consider using safe alternatives".to_string(),
        });
    }

    let quality = CodeQualityMetrics {
        complexity: calculate_complexity(contents),
        test_coverage: None,
        style_violations: count_style_violations(contents),
    };

    Ok(RepoAnalysisResult {
        vulnerabilities,
        code_quality: quality,
        suggestions,
        code: contents.to_string(), // Include the code in the result
    })
}

fn calculate_complexity(code: &str) -> f32 {
    code.lines().count() as f32 / 100.0
}

fn count_style_violations(code: &str) -> u32 {
    code.matches("unwrap()").count() as u32
}

#[query]
fn transform_response(raw_response: HttpResponse) -> HttpResponse {
    raw_response
}
