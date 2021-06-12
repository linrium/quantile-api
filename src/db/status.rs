use core::fmt;

#[derive(Debug, PartialEq)]
pub enum UpdateStatus {
    Inserted,
    Updated,
}

impl fmt::Display for UpdateStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use crate::db::UpdateStatus;

    #[test]
    fn test_to_string() {
        assert_eq!(UpdateStatus::Inserted.to_string(), "inserted");
        assert_eq!(UpdateStatus::Updated.to_string(), "updated");
    }
}
