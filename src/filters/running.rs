use super::traits::IntoQueryFilter;

pub struct Running(bool);

impl From<bool> for Running {
    fn from(value: bool) -> Self {
        Running(value)
    }
}

impl IntoQueryFilter for Running {
    fn into_query(&self) -> String {
        format!("running={}", match self.0 {
            true => "true",
            false => "false"
        })
    }
}