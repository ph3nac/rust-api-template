use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialize_deserialize() {
        let u = User {
            id: Uuid::nil(),
            name: "Alice".into(),
        };
        let json = serde_json::to_string(&u).unwrap();
        let u2: User = serde_json::from_str(&json).unwrap();
        assert_eq!(u, u2);
    }
}
