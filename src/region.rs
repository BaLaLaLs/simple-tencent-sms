pub enum Region {
    Beijing,
    Nanjing,
    Guangzhou,
    Other(String),
}
impl Region {
    pub fn get_region(&self) -> String {
        match self {
            Region::Beijing => "ap-beijing".to_string(),
            Region::Nanjing => "ap-nanjing".to_string(),
            Region::Guangzhou => "ap-guangzhou".to_string(),
            Region::Other(region) => region.to_string(),
        }
    }
}
