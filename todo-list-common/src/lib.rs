use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoItem {
    pub title: String,
    pub description: String,
    pub due_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub completed: bool,
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
