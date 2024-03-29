pub mod user;

#[derive(PartialEq, Eq, Debug, Default, Clone)]
pub struct SessionState {
    pub user_id: Option<i64>,
    pub token: Option<String>,
    // 其他需要存储的 Session 数据
}