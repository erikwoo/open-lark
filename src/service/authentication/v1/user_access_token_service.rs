use crate::core::api_req::ApiRequest;
use crate::core::api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat};
use crate::core::config::Config;
use crate::core::constants::AccessTokenType;
use crate::core::http::Transport;
use crate::core::req_option::RequestOption;
use crate::core::SDKResult;
use serde::Deserialize;
use serde::Deserialize;
use serde_json::json;

pub struct UserAccessTokenService {
    config: Config,
}

impl UserAccessTokenService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取登录用户信息
    pub async fn get(&self, code: impl ToString) -> SDKResult<BaseResponse<UserAccessToken>> {
        let json_obj = json!({
            "grant_type": "authorization_code",
            "code": code.to_string()
        });
        let json_bytes = serde_json::to_vec(&json_obj).unwrap();

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: "/open-apis/authen/v1/oidc/access_token".to_string(),
            body: json_bytes,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;

        Ok(api_resp)
    }
}

/// 登录用户信息
#[derive(Debug, Deserialize)]
pub struct UserAccessToken {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub scope: String,
}

impl ApiResponseTrait for UserAccessToken {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
