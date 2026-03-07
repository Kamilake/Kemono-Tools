use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, COOKIE};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    #[serde(default)]
    pub server: Option<String>,
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub extension: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub server: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub user: String,
    pub service: String,
    pub title: String,
    #[serde(default)]
    pub substring: Option<String>,
    #[serde(default)]
    pub published: Option<String>,
    #[serde(default)]
    pub file: Option<FileInfo>,
    #[serde(default)]
    pub attachments: Vec<Attachment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub id: u64,
    pub username: String,
}

pub struct KemonoClient {
    client: reqwest::Client,
    pub server: String,
}

impl KemonoClient {
    pub fn new(server: &str) -> Self {
        let client = reqwest::Client::builder()
            .gzip(true)
            .redirect(reqwest::redirect::Policy::limited(10))
            .build()
            .expect("Failed to build HTTP client");
        Self {
            client,
            server: server.to_string(),
        }
    }

    fn build_headers(&self, session: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("text/css"));
        if !session.is_empty() {
            let cookie_val = format!("session={}", session);
            if let Ok(v) = HeaderValue::from_str(&cookie_val) {
                headers.insert(COOKIE, v);
            }
        }
        headers
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<(LoginResponse, String), String> {
        let url = format!("{}/v1/authentication/login", self.server);
        let body = serde_json::json!({
            "username": username,
            "password": password,
        });

        let resp = self.client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Login request failed: {}", e))?;

        let mut session_cookie = String::new();
        if let Some(set_cookie) = resp.headers().get_all("set-cookie").iter().last() {
            if let Ok(val) = set_cookie.to_str() {
                for part in val.split(';') {
                    let trimmed = part.trim();
                    if trimmed.starts_with("session=") {
                        session_cookie = trimmed.strip_prefix("session=").unwrap_or("").to_string();
                        break;
                    }
                }
            }
        }

        let status = resp.status();
        // 409 = already logged in, session cookie is still returned
        if !status.is_success() && status.as_u16() != 409 {
            let text = resp.text().await.unwrap_or_default();
            return Err(format!("Login failed ({}): {}", status, text));
        }

        if status.as_u16() == 409 {
            let _ = resp.text().await;
            return Ok((LoginResponse { id: 0, username: username.to_string() }, session_cookie));
        }

        let login_resp: LoginResponse = resp.json().await
            .map_err(|e| format!("Failed to parse login response: {}", e))?;

        Ok((login_resp, session_cookie))
    }

    pub async fn get_posts(&self, session: &str, service: &str, creator_id: &str) -> Result<Vec<Post>, String> {
        let url = format!("{}/v1/{}/user/{}/posts", self.server, service, creator_id);
        let headers = self.build_headers(session);

        let resp = self.client
            .get(&url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| format!("Get posts failed: {}", e))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(format!("Get posts failed ({}): {}", status, text));
        }

        let text = resp.text().await.map_err(|e| format!("Read body failed: {}", e))?;

        // The API may return an array directly or an object with a count field
        // Try parsing as array first, then as object
        if let Ok(posts) = serde_json::from_str::<Vec<Post>>(&text) {
            return Ok(posts);
        }

        // Try as object with posts field
        #[derive(Deserialize)]
        struct PostsResponse {
            #[serde(default)]
            posts: Vec<Post>,
        }
        if let Ok(resp) = serde_json::from_str::<PostsResponse>(&text) {
            return Ok(resp.posts);
        }

        Err(format!("Failed to parse posts response: {}", &text[..text.len().min(500)]))
    }

    pub async fn get_post(&self, session: &str, service: &str, creator_id: &str, post_id: &str) -> Result<Post, String> {
        let url = format!("{}/v1/{}/user/{}/post/{}", self.server, service, creator_id, post_id);
        let headers = self.build_headers(session);

        let resp = self.client
            .get(&url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| format!("Get post failed: {}", e))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(format!("Get post failed ({}): {}", status, text));
        }

        let text = resp.text().await.map_err(|e| format!("Read body failed: {}", e))?;

        // The detail API returns { post: {...}, attachments: [...], ... }
        #[derive(Deserialize)]
        struct PostDetailResponse {
            post: Post,
            #[serde(default)]
            attachments: Vec<Attachment>,
        }

        if let Ok(detail) = serde_json::from_str::<PostDetailResponse>(&text) {
            let mut post = detail.post;
            // Merge top-level attachments into post if post.attachments is empty
            if post.attachments.is_empty() && !detail.attachments.is_empty() {
                post.attachments = detail.attachments;
            }
            return Ok(post);
        }

        // Fallback: try parsing as Post directly
        serde_json::from_str::<Post>(&text)
            .map_err(|e| format!("Failed to parse post: {}", e))
    }
}

// Tests are in tests/api_test.rs (standalone binary, no Tauri dependency)
