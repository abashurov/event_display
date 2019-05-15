use models::DisplayToken;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfoMsg {
    pub info: DisplayToken,
}
