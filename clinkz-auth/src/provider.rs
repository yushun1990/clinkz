use crate::Result;

pub trait AuthProvider: Send + Sync + 'static {
    fn find_user_by_id(&self, user_id: uuid::Uuid) -> impl Future<Output = Result<Option<User>>>;
    fn find_user_by_name(&self, user_id: uuid::Uuid) -> impl Future<Output = Result<Option<User>>>;
    fn verify_password(&self, user: &User, password: &str) -> impl Future<Output = Result<bool>>;
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub tenant_id: uuid::Uuid,
    pub username: String,
    pub scopes: Vec<String>
}
