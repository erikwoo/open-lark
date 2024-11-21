use std::collections::HashMap;

use reqwest::Method;

use crate::core::constants::AccessTokenType;

/// Requestasdf 请求结构体
#[derive(Debug, Clone, Default)]
pub struct ApiRequest {
    pub http_method: Method,
    pub api_path: String,
    pub body: Vec<u8>,
    pub query_params: HashMap<String, String>,
    pub path_params: HashMap<String, Vec<String>>,
    pub supported_access_token_types: Vec<AccessTokenType>,
    pub file: Vec<u8>,
}
