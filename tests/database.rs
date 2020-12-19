//! Integration tests for the DatabaseInterface object, this object is used for access to individual databases
extern crate aci_server;

use aci_server::database::{Database, DatabaseInterface, UserAuthentication};

#[test]
pub fn integration_test_database_read_write()
{
    let db = DatabaseInterface::new(Database::new("Database0"), chashmap::CHashMap::new());
    let user = UserAuthentication{is_authed: true, name: "user".to_string(), domain:"a_auth".to_string()};

    assert!(db.read_from_key("key", &user).is_err());
    db.write_to_key("key", serde_json::json!(0), &user).unwrap();
    assert_eq!(db.read_from_key("key", &user), Ok(serde_json::json!(0)));

    assert!(db.read_from_key("obj", &user).is_err());
    db.write_to_key("obj", serde_json::json!({"a": "b", "c": 5}), &user).unwrap();
    assert_eq!(db.read_from_key("obj", &user), Ok(serde_json::json!({"a": "b", "c": 5})));
}

#[test]
pub fn integration_test_database_read_write_lists()
{
    let db = DatabaseInterface::new(Database::new("Database0"), chashmap::CHashMap::new());
    let user = UserAuthentication{is_authed: true, name: "user".to_string(), domain:"a_auth".to_string()};

    assert!(db.read_from_key("list", &user).is_err());
    db.write_to_key("list", serde_json::json!([0, 1, "2", "3", {"a": "b", "c": 5}]), &user).unwrap();

    assert_eq!(db.read_from_key_index("list", 0, &user),
                Ok(serde_json::json!(0)));

    assert_eq!(db.read_from_key_index("list", 1, &user),
    Ok(serde_json::json!(1)));

    assert_eq!(db.read_from_key_index("list", 2, &user),
    Ok(serde_json::json!("2")));

    assert_eq!(db.read_from_key_index("list", 3, &user),
    Ok(serde_json::json!("3")));

    assert_eq!(db.read_from_key_index("list", 4, &user),
    Ok(serde_json::json!({"a": "b", "c": 5})));

    assert!(db.read_from_key_index("list", 5, &user).is_err());
}

#[test]
pub fn integration_test_database_append_lists()
{
    let db = DatabaseInterface::new(Database::new("Database0"), chashmap::CHashMap::new());
    let user = UserAuthentication{is_authed: true, name: "user".to_string(), domain:"a_auth".to_string()};

    assert!(db.read_from_key("list", &user).is_err());
    db.write_to_key("list", serde_json::json!([0, 1, "2"]), &user).unwrap();

    assert_eq!(db.read_from_key_index("list", 0, &user),
                Ok(serde_json::json!(0)));

    assert_eq!(db.read_from_key_index("list", 1, &user),
    Ok(serde_json::json!(1)));

    assert_eq!(db.read_from_key_index("list", 2, &user),
    Ok(serde_json::json!("2")));

    assert!(db.read_from_key_index("list", 3, &user).is_err());

    db.append_to_key("list", serde_json::json!("Hello World!"), &user).unwrap();

    assert_eq!(db.read_from_key_index("list", 0, &user),
                Ok(serde_json::json!(0)));

    assert_eq!(db.read_from_key_index("list", 1, &user),
    Ok(serde_json::json!(1)));

    assert_eq!(db.read_from_key_index("list", 2, &user),
    Ok(serde_json::json!("2")));

    assert_eq!(db.read_from_key_index("list", 3, &user),
    Ok(serde_json::json!("Hello World!")));

    assert!(db.read_from_key_index("list", 4, &user).is_err());
}

#[test]
pub fn integration_test_database_list_length()
{
    let db = DatabaseInterface::new(Database::new("Database0"), chashmap::CHashMap::new());
    let user = UserAuthentication{is_authed: true, name: "user".to_string(), domain:"a_auth".to_string()};

    assert!(db.read_from_key("list", &user).is_err());
    db.write_to_key("list", serde_json::json!([0, 1, "2"]), &user).unwrap();

    assert_eq!(db.get_length_from_key("list", &user), Ok(3));


    db.append_to_key("list", serde_json::json!("Hello World!"), &user).unwrap();

    assert_eq!(db.get_length_from_key("list", &user), Ok(4));
}

#[test]
pub fn integration_test_database_last_n_list()
{
    let db = DatabaseInterface::new(Database::new("Database0"), chashmap::CHashMap::new());
    let user = UserAuthentication{is_authed: true, name: "user".to_string(), domain:"a_auth".to_string()};

    assert!(db.read_from_key("list", &user).is_err());
    db.write_to_key("list", serde_json::json!([0, 1, "2", "3", 4, 5, "6"]), &user).unwrap();

    assert_eq!(db.read_last_n_from_key("list", 0, &user), Ok(serde_json::json!([])));
    assert_eq!(db.read_last_n_from_key("list", 1, &user), Ok(serde_json::json!(["6"])));
    assert_eq!(db.read_last_n_from_key("list", 2, &user), Ok(serde_json::json!([5, "6"])));
    assert_eq!(db.read_last_n_from_key("list", 3, &user), Ok(serde_json::json!([4, 5, "6"])));
    assert_eq!(db.read_last_n_from_key("list", 4, &user), Ok(serde_json::json!(["3", 4, 5, "6"])));
    assert_eq!(db.read_last_n_from_key("list", 5, &user), Ok(serde_json::json!(["2", "3", 4, 5, "6"])));
    assert_eq!(db.read_last_n_from_key("list", 6, &user), Ok(serde_json::json!([1, "2", "3", 4, 5, "6"])));
    assert_eq!(db.read_last_n_from_key("list", 7, &user), Ok(serde_json::json!([0, 1, "2", "3", 4, 5, "6"])));
    assert_eq!(db.read_last_n_from_key("list", 8, &user), Ok(serde_json::json!([0, 1, "2", "3", 4, 5, "6"])));
}