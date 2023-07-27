#[derive(Clone)]
pub struct Person {
    id: String,
    name: String,
    birthdate: String,
    nationality: String,
    social_media_followers: i32,
    languages: Vec<String>,
}
