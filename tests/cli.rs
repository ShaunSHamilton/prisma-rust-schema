mod schema;
use bson::oid::ObjectId;
use schema::*;
use serde_json::json;

#[test]
fn user_model() {
    let user_json = json!({
        "_id": {
            "$oid": "507f1f77bcf86cd799439011"
        },
        "email": "test@test.com",
        "name": "John Doe",
        "permission": "USER",
        "createdAt": 1234567890,
        "status": {
            "active": true,
            "lastLogin": 1234567890,
        },
        "badCase": [],
        "posts": []
    });

    let user: User = serde_json::from_value(user_json).unwrap();
    assert_eq!(
        user.id,
        ObjectId::parse_str("507f1f77bcf86cd799439011").unwrap()
    );
    assert_eq!(user.email, "test@test.com".to_string());
    assert_eq!(
        user.username,
        Some("John Doe".to_string()),
        "name is renamed to username"
    );
    assert_eq!(user.permission, Permission::USER);
    assert_eq!(user.created_at, 1234567890usize);
    assert_eq!(
        user.status,
        json!({"active": true, "lastLogin": 1234567890})
    );
    assert_eq!(user.bad_case, vec![]);
    assert_eq!(user.posts, vec![]);
}

#[test]
fn post_model() {
    let post_json = json!({
        "_id": {
            "$oid": "507f1f77bcf86cd799439011"
        },
        "content": {
            "text": "Hello world",
            "images": [
                "https://example.com/image1.jpg",
                "https://example.com/image2.jpg"
            ]
        },
        "authorId": {
            "$oid": "507f1f77bcf86cd799439012"
        }
    });

    let post: Post = serde_json::from_value(post_json).unwrap();
    assert_eq!(
        post.id,
        ObjectId::parse_str("507f1f77bcf86cd799439011").unwrap()
    );
    assert_eq!(post.content.text, "Hello world".to_string());
    assert_eq!(
        post.content.images,
        vec![
            "https://example.com/image1.jpg".to_string(),
            "https://example.com/image2.jpg".to_string()
        ]
    );
    assert_eq!(
        post.author_id,
        ObjectId::parse_str("507f1f77bcf86cd799439012").unwrap()
    );
}

#[test]
fn bad_case_model() {
    let bad_case_json = json!({
        "_id": {
            "$oid": "507f1f77bcf86cd799439011"
        },
        "userId": {
            "$oid": "507f1f77bcf86cd799439012"
        }
    });

    let bad_case: BadCase = serde_json::from_value(bad_case_json).unwrap();
    assert_eq!(
        bad_case.id,
        ObjectId::parse_str("507f1f77bcf86cd799439011").unwrap()
    );
    assert_eq!(
        bad_case.user_id,
        ObjectId::parse_str("507f1f77bcf86cd799439012").unwrap()
    );
}

#[test]
fn content_model() {
    let content_json = json!({
        "text": "Hello world",
        "images": [
            "https://example.com/image1.jpg",
            "https://example.com/image2.jpg"
        ]
    });

    let content: Content = serde_json::from_value(content_json).unwrap();
    assert_eq!(content.text, "Hello world".to_string());
    assert_eq!(
        content.images,
        vec![
            "https://example.com/image1.jpg".to_string(),
            "https://example.com/image2.jpg".to_string()
        ]
    );
}

#[test]
fn permission_model() {
    let user_permission = Permission::USER;
    let admin_permission = Permission::ADMIN;

    assert_eq!(user_permission, Permission::USER);
    assert_eq!(admin_permission, Permission::ADMIN);
}
