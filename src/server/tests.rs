//! Somewhere Between Unit and Integration Tests

use crate::server;
use crate::commands;
use crate::args;

use structopt::StructOpt;

use serde_json::json;

#[test]
pub fn test_read_write_disk()
{
    let mut opt = args::Arguments::from_args();
    opt.path = std::path::PathBuf::from("test-databases");

    let server = server::Server::new(&opt).unwrap();
    let mut conn = server::ServerInterface::new(&std::sync::Arc::new(server));
    conn.fake_auth();

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "read_from_disk", "db_key": "command"})).unwrap()), Ok(None));
    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "write_to_disk", "db_key": "command"})).unwrap()), Ok(None));
}

#[test]
pub fn test_list_databases()
{
    let mut opt = args::Arguments::from_args();
    opt.path = std::path::PathBuf::from("test-databases");

    let server = server::Server::new(&opt).unwrap();
    let mut conn = server::ServerInterface::new(&std::sync::Arc::new(server));
    conn.fake_auth();

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "read_from_disk", "db_key": "command"})).unwrap()), Ok(None));
    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "list_databases", "db_key": "command"})).unwrap()),
                Ok(Some(json!({"cmd": "ldResp", "msg": ["load_cell_known_mass", "test_begin", "test_end"]}))));
}

#[test]
pub fn test_get_value()
{
    let mut opt = args::Arguments::from_args();
    opt.path = std::path::PathBuf::from("test-databases");

    let server = server::Server::new(&opt).unwrap();
    let mut conn = server::ServerInterface::new(&std::sync::Arc::new(server));
    conn.fake_auth();

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "read_from_disk", "db_key": "command"})).unwrap()), Ok(None));
    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_value", "db_key": "command", "key": "test_begin"})).unwrap()),
                Ok(Some(json!({"cmd": "getResp", "key": "test_begin", "val": json!("True"), "db_key": "command"}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_value", "db_key": "command", "key": "test_end"})).unwrap()),
                Ok(Some(json!({"cmd": "getResp", "key": "test_end", "val": json!("False"), "db_key": "command"}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_value", "db_key": "command", "key": "load_cell_known_mass"})).unwrap()),
                Ok(Some(json!({"cmd": "getResp", "key": "load_cell_known_mass", "val": json!(0), "db_key": "command"}))));
}

#[test]
pub fn test_set_value()
{
    let mut opt = args::Arguments::from_args();
    opt.path = std::path::PathBuf::from("test-databases");

    let server = server::Server::new(&opt).unwrap();
    let mut conn = server::ServerInterface::new(&std::sync::Arc::new(server));
    conn.fake_auth();

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "read_from_disk", "db_key": "command"})).unwrap()), Ok(None));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "set_value", "db_key": "command", "key": "test_begin", "val": "False"})).unwrap()),
                Ok(Some(json!(
                    {"cmd": "setResp", "msg": "command[test_begin]=\"False\"", "db_key": "command", "val": "False", "key": "test_begin"}
                ))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "set_value", "db_key": "command", "key": "test_end", "val": "True"})).unwrap()),
                Ok(Some(json!(
                    {"cmd": "setResp", "msg": "command[test_end]=\"True\"", "db_key": "command", "val": "True", "key": "test_end"}
                ))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "set_value", "db_key": "command", "key": "load_cell_known_mass", "val": 100})).unwrap()),
                Ok(Some(json!(
                    {"cmd": "setResp", "msg": "command[load_cell_known_mass]=100", "db_key": "command", "val": 100, "key": "load_cell_known_mass"}
                ))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_value", "db_key": "command", "key": "test_begin"})).unwrap()),
                Ok(Some(json!({"cmd": "getResp", "key": "test_begin", "val": json!("False"), "db_key": "command"}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_value", "db_key": "command", "key": "test_end"})).unwrap()),
                Ok(Some(json!({"cmd": "getResp", "key": "test_end", "val": json!("True"), "db_key": "command"}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_value", "db_key": "command", "key": "load_cell_known_mass"})).unwrap()),
                Ok(Some(json!({"cmd": "getResp", "key": "load_cell_known_mass", "val": json!(100), "db_key": "command"}))));
}

#[test]
pub fn test_get_index()
{
    let mut opt = args::Arguments::from_args();
    opt.path = std::path::PathBuf::from("test-databases");

    let server = server::Server::new(&opt).unwrap();
    let mut conn = server::ServerInterface::new(&std::sync::Arc::new(server));
    conn.fake_auth();

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "read_from_disk", "db_key": "test"})).unwrap()), Ok(None));
    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_index", "db_key": "test", "key": "list0", "index": 0})).unwrap()),
                Ok(Some(json!({"cmd": "get_indexResp", "key": "list0", "msg": json!(0), "db_key": "test", "index": 0}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_index", "db_key": "test", "key": "list0", "index": 1})).unwrap()),
                Ok(Some(json!({"cmd": "get_indexResp", "key": "list0", "msg": json!(1), "db_key": "test", "index": 1}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_index", "db_key": "test", "key": "list0", "index": 2})).unwrap()),
                Ok(Some(json!({"cmd": "get_indexResp", "key": "list0", "msg": json!("2"), "db_key": "test", "index": 2}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_index", "db_key": "test", "key": "list0", "index": 3})).unwrap()),
                Ok(Some(json!({"cmd": "get_indexResp", "key": "list0", "msg": json!("3"), "db_key": "test", "index": 3}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_index", "db_key": "test", "key": "list0", "index": 4})).unwrap()),
                Ok(Some(json!({"cmd": "get_indexResp", "key": "list0", "msg": json!(true), "db_key": "test", "index": 4}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_index", "db_key": "test", "key": "list0", "index": 5})).unwrap()),
                Ok(Some(json!({"cmd": "get_indexResp", "key": "list0", "msg": json!(4.0), "db_key": "test", "index": 5}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_index", "db_key": "test", "key": "list0", "index": 6})).unwrap()),
                Ok(Some(json!({"cmd": "get_indexResp", "key": "list0", "msg": json!(false), "db_key": "test", "index": 6}))));
}

#[test]
pub fn test_set_index()
{
    let mut opt = args::Arguments::from_args();
    opt.path = std::path::PathBuf::from("test-databases");

    let server = server::Server::new(&opt).unwrap();
    let mut conn = server::ServerInterface::new(&std::sync::Arc::new(server));
    conn.fake_auth();

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "read_from_disk", "db_key": "test"})).unwrap()), Ok(None));
    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "set_index", "db_key": "test", "key": "list0", "index": 0, "val": json!(1)})).unwrap()),
                Ok(Some(json!({"cmd": "set_indexResp", "key": "list0", "msg": json!(1), "db_key": "test", "index": 0}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "set_index", "db_key": "test", "key": "list0", "index": 1, "val": json!(2)})).unwrap()),
                Ok(Some(json!({"cmd": "set_indexResp", "key": "list0", "msg": json!(2), "db_key": "test", "index": 1}))));
                
    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "set_index", "db_key": "test", "key": "list0", "index": 2, "val": json!(3)})).unwrap()),
                Ok(Some(json!({"cmd": "set_indexResp", "key": "list0", "msg": json!(3), "db_key": "test", "index": 2}))));
                
    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "set_index", "db_key": "test", "key": "list0", "index": 3, "val": json!(4)})).unwrap()),
                Ok(Some(json!({"cmd": "set_indexResp", "key": "list0", "msg": json!(4), "db_key": "test", "index": 3}))));
                
    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "set_index", "db_key": "test", "key": "list0", "index": 4, "val": json!(5)})).unwrap()),
                Ok(Some(json!({"cmd": "set_indexResp", "key": "list0", "msg": json!(5), "db_key": "test", "index": 4}))));
                
    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "set_index", "db_key": "test", "key": "list0", "index": 5, "val": json!(6)})).unwrap()),
                Ok(Some(json!({"cmd": "set_indexResp", "key": "list0", "msg": json!(6), "db_key": "test", "index": 5}))));
                
    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "set_index", "db_key": "test", "key": "list0", "index": 6, "val": json!(7)})).unwrap()),
                Ok(Some(json!({"cmd": "set_indexResp", "key": "list0", "msg": json!(7), "db_key": "test", "index": 6}))));
}

#[test]
pub fn test_append()
{
    let mut opt = args::Arguments::from_args();
    opt.path = std::path::PathBuf::from("test-databases");

    let server = server::Server::new(&opt).unwrap();
    let mut conn = server::ServerInterface::new(&std::sync::Arc::new(server));
    conn.fake_auth();

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "read_from_disk", "db_key": "test"})).unwrap()), Ok(None));
    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "append_list", "db_key": "test", "key": "list0", "val": json!(1)})).unwrap()),
                Ok(Some(json!({"cmd": "app_indexResp", "key": "list0", "msg": json!(1), "db_key": "test", "index": 7}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "append_list", "db_key": "test", "key": "list0", "val": json!(5)})).unwrap()),
                Ok(Some(json!({"cmd": "app_indexResp", "key": "list0", "msg": json!(5), "db_key": "test", "index": 8}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "append_list", "db_key": "test", "key": "list0", "val": json!(true)})).unwrap()),
                Ok(Some(json!({"cmd": "app_indexResp", "key": "list0", "msg": json!(true), "db_key": "test", "index": 9}))));
}

#[test]
pub fn test_get_length()
{
    let mut opt = args::Arguments::from_args();
    opt.path = std::path::PathBuf::from("test-databases");

    let server = server::Server::new(&opt).unwrap();
    let mut conn = server::ServerInterface::new(&std::sync::Arc::new(server));
    conn.fake_auth();

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "read_from_disk", "db_key": "test"})).unwrap()), Ok(None));
    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_list_length", "db_key": "test", "key": "list0"})).unwrap()),
                Ok(Some(json!({"cmd": "get_list_lengthResp", "key": "list0", "msg": json!(7), "db_key": "test"}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_list_length", "db_key": "test", "key": "list1"})).unwrap()),
                Ok(Some(json!({"cmd": "get_list_lengthResp", "key": "list1", "msg": json!(0), "db_key": "test"}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_list_length", "db_key": "test", "key": "list2"})).unwrap()),
                Ok(Some(json!({"cmd": "get_list_lengthResp", "key": "list2", "msg": json!(3), "db_key": "test"}))));
}

#[test]
pub fn test_get_recent()
{
    let mut opt = args::Arguments::from_args();
    opt.path = std::path::PathBuf::from("test-databases");

    let server = server::Server::new(&opt).unwrap();
    let mut conn = server::ServerInterface::new(&std::sync::Arc::new(server));
    conn.fake_auth();

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "read_from_disk", "db_key": "test"})).unwrap()), Ok(None));
    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_recent", "db_key": "test", "key": "list0", "num": 3})).unwrap()),
                Ok(Some(json!({"cmd": "get_recentResp", "key": "list0", "msg": json!([false, 4.0, true]), "db_key": "test", "num": 3}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_recent", "db_key": "test", "key": "list0", "num": 5})).unwrap()),
                Ok(Some(json!({"cmd": "get_recentResp", "key": "list0", "msg": json!([false, 4.0, true, "3", "2"]), "db_key": "test", "num": 5}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_recent", "db_key": "test", "key": "list0", "num": 10})).unwrap()),
                Ok(Some(json!({"cmd": "get_recentResp", "key": "list0", "msg": json!([false, 4.0, true, "3", "2", 1, 0]), "db_key": "test", "num": 10}))));
}

#[test]
pub fn test_create_database()
{
    let mut opt = args::Arguments::from_args();
    opt.path = std::path::PathBuf::from("test-databases");

    let server = server::Server::new(&opt).unwrap();
    let mut conn = server::ServerInterface::new(&std::sync::Arc::new(server));
    conn.fake_auth();

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "create_database", "db_key": "database0"})).unwrap()), Ok(None));
    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "create_database", "db_key": "database1"})).unwrap()), Ok(None));
    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "create_database", "db_key": "database2"})).unwrap()), Ok(None));
}