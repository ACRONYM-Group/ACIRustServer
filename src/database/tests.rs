use serde_json::json;

use super::Database;


#[test]
pub fn test_database_creation()
{
    let db0 = Database::new("Database 0");
    let db1 = Database::new("");
    let db2 = Database::new("This is a very very very long database name, nobody in their right mind would actually create a database with a title like this as it would take a large amount of bandwidth just to send such a name. However, this is not actually going to be used in a serious setting, and is only being used to test if the database can handle very long names");
    let db3 = Database::new("ði ıntəˈnæʃənəl fəˈnɛtık əsoʊsiˈeıʃn");

    assert_eq!(db0.get_name().as_str(), "Database 0");
    assert_eq!(db1.get_name().as_str(), "");
    assert_eq!(db2.get_name().as_str(), "This is a very very very long database name, nobody in their right mind would actually create a database with a title like this as it would take a large amount of bandwidth just to send such a name. However, this is not actually going to be used in a serious setting, and is only being used to test if the database can handle very long names");
    assert_eq!(db3.get_name().as_str(), "ði ıntəˈnæʃənəl fəˈnɛtık əsoʊsiˈeıʃn");

    assert_eq!(db0.get_number_of_keys(), 0);
    assert_eq!(db1.get_number_of_keys(), 0);
    assert_eq!(db2.get_number_of_keys(), 0);
    assert_eq!(db3.get_number_of_keys(), 0);
}

#[test]
pub fn test_database_read_write()
{
    let db = Database::new("Database");

    let temp_json: serde_json::Value = serde_json::from_str("{\"key\": 0, \"other\": {\"a\": 0, \"b\": 1, \"c\": 2}, \"another\": [0, 1, 2, \"3\", \"4\"]}").unwrap();
    
    let (d0, d1, d2) = match temp_json
    {
        serde_json::Value::Object(obj) =>
        {
            (obj.get("key").unwrap().clone(), obj.get("other").unwrap().clone(), obj.get("another").unwrap().clone())
        },
        _ => panic!("Bad Input JSON")
    };

    assert!(db.read("key").is_err());
    assert!(db.read("other").is_err());
    assert!(db.read("another").is_err());

    db.write("key", d0.clone()).unwrap();
    db.write("other", d1.clone()).unwrap();
    db.write("another", d2.clone()).unwrap();

    assert_eq!(db.read("key"), Ok(d0.clone()));
    assert_eq!(db.read("other"), Ok(d1.clone()));
    assert_eq!(db.read("another"), Ok(d2.clone()));

    db.write("key", d2.clone()).unwrap();
    db.write("other", d0.clone()).unwrap();
    db.write("another", d1.clone()).unwrap();
    
    assert_eq!(db.read("key"), Ok(d2.clone()));
    assert_eq!(db.read("other"), Ok(d0.clone()));
    assert_eq!(db.read("another"), Ok(d1.clone()));
    
    db.write("other", d1.clone()).unwrap();

    assert_eq!(db.read("key"), Ok(d2.clone()));
    assert_eq!(db.read("other"), Ok(d1.clone()));
    assert_eq!(db.read("another"), Ok(d1.clone()));
}

#[test]
pub fn test_database_read_write_index()
{
    let db = Database::new("Database");

    let temp_json: serde_json::Value = serde_json::from_str("{\"key\": 0, \"other\": {\"a\": 0, \"b\": 1, \"c\": 2}, \"another\": [0, 1, 2, \"3\", \"4\"]}").unwrap();
    
    let (_, nonlst, lst) = match temp_json
    {
        serde_json::Value::Object(obj) =>
        {
            (obj.get("key").unwrap().clone(), obj.get("other").unwrap().clone(), obj.get("another").unwrap().clone())
        },
        _ => panic!("Bad Input JSON")
    };

    db.write("key", lst.clone()).unwrap();
    db.write("nonlst", nonlst.clone()).unwrap();

    assert!(db.read_index("nonlst", 0).is_err());

    assert_eq!(db.read_index("key", 0), Ok(json!("0")));
    assert_eq!(db.read_index("key", 1), Ok(json!("1")));
    assert_eq!(db.read_index("key", 2), Ok(json!("2")));
    assert_eq!(db.read_index("key", 3), Ok(json!("\"3\"")));
    assert_eq!(db.read_index("key", 4), Ok(json!("\"4\"")));
    assert!(db.read_index("key", 5).is_err());

    db.write_index("key", 0, json!("\"0\"")).unwrap();
    db.write_index("key", 1, json!("\"1\"")).unwrap();
    db.write_index("key", 2, json!("\"2\"")).unwrap();
    db.write_index("key", 3, json!("3")).unwrap();
    db.write_index("key", 4, json!("4")).unwrap();

    assert_eq!(db.read_index("key", 0), Ok(json!("\"0\"")));
    assert_eq!(db.read_index("key", 1), Ok(json!("\"1\"")));
    assert_eq!(db.read_index("key", 2), Ok(json!("\"2\"")));
    assert_eq!(db.read_index("key", 3), Ok(json!("3")));
    assert_eq!(db.read_index("key", 4), Ok(json!("4")));
    assert!(db.read_index("key", 5).is_ok());

    db.write_index("key", 5, json!("5")).unwrap();

    assert_eq!(db.read_index("key", 0), Ok(json!("\"0\"")));
    assert_eq!(db.read_index("key", 1), Ok(json!("\"1\"")));
    assert_eq!(db.read_index("key", 2), Ok(json!("\"2\"")));
    assert_eq!(db.read_index("key", 3), Ok(json!("3")));
    assert_eq!(db.read_index("key", 4), Ok(json!("4")));
    assert_eq!(db.read_index("key", 5), Ok(json!("5")));
    assert!(db.read_index("key", 6).is_err());

    db.write_index("key", 8, json!("8")).unwrap();

    assert_eq!(db.read_index("key", 0), Ok(json!("\"0\"")));
    assert_eq!(db.read_index("key", 1), Ok(json!("\"1\"")));
    assert_eq!(db.read_index("key", 2), Ok(json!("\"2\"")));
    assert_eq!(db.read_index("key", 3), Ok(json!("3")));
    assert_eq!(db.read_index("key", 4), Ok(json!("4")));
    assert_eq!(db.read_index("key", 5), Ok(json!("5")));
    assert_eq!(db.read_index("key", 6), Ok(json!("null")));
    assert_eq!(db.read_index("key", 7), Ok(json!("null")));
    assert_eq!(db.read_index("key", 8), Ok(json!("8")));
    assert!(db.read_index("key", 9).is_err());
}

#[test]
pub fn test_database_get_length()
{
    let db = Database::new("Database");

    assert_eq!(db.get_number_of_keys(), 0);

    db.write("key", json!("0")).unwrap();
    assert_eq!(db.get_number_of_keys(), 1);

    db.write("key_1", json!("42")).unwrap();
    assert_eq!(db.get_number_of_keys(), 2);

    db.write("key_2", json!("-834")).unwrap();
    assert_eq!(db.get_number_of_keys(), 3);

    db.write("key", json!("8")).unwrap();
    assert_eq!(db.get_number_of_keys(), 3);

    db.write("key_3", json!("21")).unwrap();
    assert_eq!(db.get_number_of_keys(), 4);

    db.write("key_1", json!("3621")).unwrap();
    assert_eq!(db.get_number_of_keys(), 4);
}

#[test]
pub fn test_database_get_length_index()
{
    let db = Database::new("Database");

    db.write("key", json!([])).unwrap();
    assert_eq!(db.get_length("key"), Ok(0));

    db.write("key", json!([0, 5, "hello world"])).unwrap();
    assert_eq!(db.get_length("key"), Ok(3));

    db.write("key", json!([0, 5, 8, 23, -190246.456, 9235])).unwrap();
    assert_eq!(db.get_length("key"), Ok(6));
}

#[test]
pub fn test_database_append()
{
    let db = Database::new("Database");

    db.write("key", json!([])).unwrap();
    assert_eq!(db.get_length("key"), Ok(0));
    assert!(db.read_index("key", 0).is_err());

    db.append("key", json!("0")).unwrap();
    assert_eq!(db.get_length("key"), Ok(1));
    assert_eq!(db.read_index("key", 0), Ok(json!("0")));
    assert!(db.read_index("key", 1).is_err());

    db.append("key", json!("3")).unwrap();
    assert_eq!(db.get_length("key"), Ok(2));
    assert_eq!(db.read_index("key", 0), Ok(json!("0")));
    assert_eq!(db.read_index("key", 1), Ok(json!("3")));
    assert!(db.read_index("key", 2).is_err());

    db.append("key", json!("\"Hello World\"")).unwrap();
    assert_eq!(db.get_length("key"), Ok(3));
    assert_eq!(db.read_index("key", 0), Ok(json!("0")));
    assert_eq!(db.read_index("key", 1), Ok(json!("3")));
    assert_eq!(db.read_index("key", 2), Ok(json!("\"Hello World\"")));
    assert!(db.read_index("key", 3).is_err());
}