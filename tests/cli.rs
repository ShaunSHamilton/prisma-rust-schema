use bson::doc;
mod schema;
use schema::*;

#[test]
fn rename() {
    let user_json = doc! {
        "_id": {
            "$oid": "507f1f77bcf86cd799439011"
        },
        "name": "John Doe",
        "email": "test@test.com",
        "permission": "USER",
        "createdAt": 1234567890,
        "status": {
            "active": true,
            "lastLogin": 1234567890,
        },
        "badCase": [],
        "posts": []
    };

    let user: User = bson::from_document(user_json).unwrap();
    assert_eq!(
        user.id,
        bson::oid::ObjectId::parse_str("507f1f77bcf86cd799439011").unwrap()
    );
    assert_eq!(user.username, Some("John Doe".to_string()));
    assert_eq!(user.email, "test@test.com".to_string());
    assert_eq!(user.permission, Permission::USER);
    assert_eq!(user.created_at, 1234567890usize);
}
