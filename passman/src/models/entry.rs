use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

#[derive(Serialize, Deserialize, Clone, Zeroize)]
#[zeroize(drop)]
pub struct Entry {
    pub name: String,
    pub username: String,
    pub password: String,
    pub url: String,
    pub notes: String,
}
