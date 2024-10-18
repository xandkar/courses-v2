#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Echo {
    pub text: String,
}
