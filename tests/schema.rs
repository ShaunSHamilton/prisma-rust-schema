#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
    pub email: String,
    #[serde(rename = "name")]
    pub username: Option<String>,
    pub permission: Permission,
    #[serde(rename = "createdAt")]
    pub created_at: usize,
    pub status: serde_json::Value,
    #[serde(rename = "badCase")]
    pub bad_case: Vec<BadCase>,
    pub posts: Vec<Post>,
}
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct Post {
    #[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
    pub content: Content,
    #[serde(rename = "authorId")]
    pub author_id: bson::oid::ObjectId,
}
#[doc = "A model with a bad casing"]
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct BadCase {
    #[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
    #[serde(rename = "userId")]
    pub user_id: bson::oid::ObjectId,
}
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct Content {
    pub text: String,
    pub images: Vec<String>,
}
#[doc = "Permission for `User`"]
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize, PartialEq)]
pub enum Permission {
    #[doc = "A common user"]
    USER,
    #[doc = "All writes user"]
    ADMIN,
}
