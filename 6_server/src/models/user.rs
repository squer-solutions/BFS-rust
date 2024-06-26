use email_address::EmailAddress;
use nutype::nutype;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: Username,
    pub email: EmailAddress,
}

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 20, len_char_min = 3),
    derive(Debug, Clone, Serialize, Deserialize, Display, PartialEq)
)]
pub struct Username(String);

#[derive(Clone, Debug, Deserialize)]
pub struct CreateUser {
    pub name: Username,
    pub email: EmailAddress,
}
