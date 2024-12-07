pub use auth::*;
pub use user_access_token_service::*;
mod auth;
mod user_access_token_service;

pub struct V1 {
    /// 身份验证
    pub user_info: UserInfoService,
    pub user_access_token: UserAccessTokenService,
}

impl V1 {
    pub fn new(config: crate::core::config::Config) -> Self {
        Self {
            user_access_token: UserAccessTokenService::new(config.clone()),
            user_info: UserInfoService::new(config.clone()),
        }
    }
}
