use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::User;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserOut {
    display_name: String,
    initiated_at: String,
}

impl<'user> From<&'user User> for UserOut {
    fn from(User { display_name, initiated_at, ..}: &'user User) -> Self {
        Self {
            display_name: display_name.to_owned(),
            initiated_at: initiated_at.to_string()
        }
    }
}
