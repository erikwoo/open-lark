pub use auth::*;

mod auth;

pub struct V1 {
    /// 身份验证
    pub user_info: UserInfoService,
    pub user_access_token: UserAccessTokenService,
}

impl V1 {
    pub fn new(config: crate::core::config::Config) -> Self {
        Self {
            user_info: UserInfoService::new(config),
            user_access_token: UserAccessTokenService::new(config),
        }
    }
}
