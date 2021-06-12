use core::fmt;

#[derive(Debug)]
pub enum UpdateStatus {
    Inserted,
    Updated,
}

impl fmt::Display for UpdateStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}
