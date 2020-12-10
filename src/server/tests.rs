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

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "read_from_disk", "db_key": "command"})).unwrap()),
                Ok(Some(json!({"cmd": "read_from_disk", "mode": "ok", "msg": "", "db_key": "command"}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "write_to_disk", "db_key": "command"})).unwrap()),
                Ok(Some(json!({"cmd": "write_to_disk", "mode": "ok", "msg": "", "db_key": "command"}))));
}

#[test]
pub fn test_list_databases()
{
    let mut opt = args::Arguments::from_args();
    opt.path = std::path::PathBuf::from("test-databases");

    let server = server::Server::new(&opt).unwrap();
    let mut conn = server::ServerInterface::new(&std::sync::Arc::new(server));
    conn.fake_auth();

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "read_from_disk", "db_key": "command"})).unwrap()),
                Ok(Some(json!({"cmd": "read_from_disk", "mode": "ok", "msg": "", "db_key": "command"}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "list_keys", "db_key": "command"})).unwrap()),
                Ok(Some(json!({"cmd": "list_keys", "mode": "ok", "msg": "", "db_key": "command", "val": ["load_cell_known_mass", "test_begin", "test_end"]}))));
}

#[test]
pub fn test_get_value()
{
    let mut opt = args::Arguments::from_args();
    opt.path = std::path::PathBuf::from("test-databases");

    let server = server::Server::new(&opt).unwrap();
    let mut conn = server::ServerInterface::new(&std::sync::Arc::new(server));
    conn.fake_auth();

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "read_from_disk", "db_key": "command"})).unwrap()),
                Ok(Some(json!({"cmd": "read_from_disk", "mode": "ok", "msg": "", "db_key": "command"}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_value", "db_key": "command", "key": "test_begin"})).unwrap()),
                Ok(Some(json!({"cmd": "get_value", "mode": "ok", "msg": "", "db_key": "command", "key": "test_begin", "val": "True"}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_value", "db_key": "command", "key": "test_end"})).unwrap()),
                Ok(Some(json!({"cmd": "get_value", "mode": "ok", "msg": "", "db_key": "command", "key": "test_end", "val": "False"}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_value", "db_key": "command", "key": "load_cell_known_mass"})).unwrap()),
                Ok(Some(json!({"cmd": "get_value", "mode": "ok", "msg": "", "db_key": "command", "key": "load_cell_known_mass", "val": 0}))));
}

#[test]
pub fn test_set_value()
{
    let mut opt = args::Arguments::from_args();
    opt.path = std::path::PathBuf::from("test-databases");

    let server = server::Server::new(&opt).unwrap();
    let mut conn = server::ServerInterface::new(&std::sync::Arc::new(server));
    conn.fake_auth();

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "read_from_disk", "db_key": "command"})).unwrap()),
                Ok(Some(json!({"cmd": "read_from_disk", "mode": "ok", "msg": "", "db_key": "command"}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "set_value", "db_key": "command", "key": "test_begin", "val": "False"})).unwrap()),
                Ok(Some(json!(
                    {"cmd": "set_value", "mode": "ok", "msg": "", "db_key": "command", "key": "test_begin"}
                ))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "set_value", "db_key": "command", "key": "test_end", "val": "True"})).unwrap()),
                Ok(Some(json!(
                    {"cmd": "set_value", "mode": "ok", "msg": "", "db_key": "command", "key": "test_end"}
                ))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "set_value", "db_key": "command", "key": "load_cell_known_mass", "val": 100})).unwrap()),
                Ok(Some(json!(
                    {"cmd": "set_value", "mode": "ok", "msg": "", "db_key": "command", "key": "load_cell_known_mass"}
                ))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_value", "db_key": "command", "key": "test_begin"})).unwrap()),
                Ok(Some(json!({"cmd": "get_value", "mode": "ok", "msg": "", "db_key": "command", "key": "test_begin", "val": "False"}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_value", "db_key": "command", "key": "test_end"})).unwrap()),
                Ok(Some(json!({"cmd": "get_value", "mode": "ok", "msg": "", "db_key": "command", "key": "test_end", "val": "True"}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_value", "db_key": "command", "key": "load_cell_known_mass"})).unwrap()),
                Ok(Some(json!({"cmd": "get_value", "mode": "ok", "msg": "", "db_key": "command", "key": "load_cell_known_mass", "val": 100}))));
}

#[test]
pub fn test_get_index()
{
    let mut opt = args::Arguments::from_args();
    opt.path = std::path::PathBuf::from("test-databases");

    let server = server::Server::new(&opt).unwrap();
    let mut conn = server::ServerInterface::new(&std::sync::Arc::new(server));
    conn.fake_auth();

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "read_from_disk", "db_key": "test"})).unwrap()), 
                Ok(Some(json!({"cmd": "read_from_disk", "mode": "ok", "msg": "", "db_key": "test"}))));
                
    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_index", "db_key": "test", "key": "list0", "index": 0})).unwrap()),
                Ok(Some(json!({"cmd": "get_index", "mode": "ok", "msg": "", "db_key": "test", "key": "list0", "index": 0, "val": 0}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_index", "db_key": "test", "key": "list0", "index": 1})).unwrap()),
                Ok(Some(json!({"cmd": "get_index", "mode": "ok", "msg": "", "db_key": "test", "key": "list0", "index": 1, "val": 1}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_index", "db_key": "test", "key": "list0", "index": 2})).unwrap()),
                Ok(Some(json!({"cmd": "get_index", "mode": "ok", "msg": "", "db_key": "test", "key": "list0", "index": 2, "val": "2"}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_index", "db_key": "test", "key": "list0", "index": 3})).unwrap()),
                Ok(Some(json!({"cmd": "get_index", "mode": "ok", "msg": "", "db_key": "test", "key": "list0", "index": 3, "val": "3"}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_index", "db_key": "test", "key": "list0", "index": 4})).unwrap()),
                Ok(Some(json!({"cmd": "get_index", "mode": "ok", "msg": "", "db_key": "test", "key": "list0", "index": 4, "val": true}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_index", "db_key": "test", "key": "list0", "index": 5})).unwrap()),
                Ok(Some(json!({"cmd": "get_index", "mode": "ok", "msg": "", "db_key": "test", "key": "list0", "index": 5, "val": 4.0}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_index", "db_key": "test", "key": "list0", "index": 6})).unwrap()),
                Ok(Some(json!({"cmd": "get_index", "mode": "ok", "msg": "", "db_key": "test", "key": "list0", "index": 6, "val": false}))));
}

#[test]
pub fn test_set_index()
{
    let mut opt = args::Arguments::from_args();
    opt.path = std::path::PathBuf::from("test-databases");

    let server = server::Server::new(&opt).unwrap();
    let mut conn = server::ServerInterface::new(&std::sync::Arc::new(server));
    conn.fake_auth();

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "read_from_disk", "db_key": "test"})).unwrap()), 
                Ok(Some(json!({"cmd": "read_from_disk", "mode": "ok", "msg": "", "db_key": "test"}))));
    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "set_index", "db_key": "test", "key": "list0", "index": 0, "val": json!(1)})).unwrap()),
                Ok(Some(json!({"cmd": "set_index", "mode": "ok", "msg": "", "db_key": "test", "key": "list0", "index": 0}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "set_index", "db_key": "test", "key": "list0", "index": 1, "val": json!(2)})).unwrap()),
                Ok(Some(json!({"cmd": "set_index", "mode": "ok", "msg": "", "db_key": "test", "key": "list0", "index": 1}))));
                
    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "set_index", "db_key": "test", "key": "list0", "index": 2, "val": json!(3)})).unwrap()),
                Ok(Some(json!({"cmd": "set_index", "mode": "ok", "msg": "", "db_key": "test", "key": "list0", "index": 2}))));
                
    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "set_index", "db_key": "test", "key": "list0", "index": 3, "val": json!(4)})).unwrap()),
                Ok(Some(json!({"cmd": "set_index", "mode": "ok", "msg": "", "db_key": "test", "key": "list0", "index": 3}))));
                
    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "set_index", "db_key": "test", "key": "list0", "index": 4, "val": json!(5)})).unwrap()),
                Ok(Some(json!({"cmd": "set_index", "mode": "ok", "msg": "", "db_key": "test", "key": "list0", "index": 4}))));
                
    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "set_index", "db_key": "test", "key": "list0", "index": 5, "val": json!(6)})).unwrap()),
                Ok(Some(json!({"cmd": "set_index", "mode": "ok", "msg": "", "db_key": "test", "key": "list0", "index": 5}))));
                
    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "set_index", "db_key": "test", "key": "list0", "index": 6, "val": json!(7)})).unwrap()),
                Ok(Some(json!({"cmd": "set_index", "mode": "ok", "msg": "", "db_key": "test", "key": "list0", "index": 6}))));
}

#[test]
pub fn test_append()
{
    let mut opt = args::Arguments::from_args();
    opt.path = std::path::PathBuf::from("test-databases");

    let server = server::Server::new(&opt).unwrap();
    let mut conn = server::ServerInterface::new(&std::sync::Arc::new(server));
    conn.fake_auth();

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "read_from_disk", "db_key": "test"})).unwrap()), 
                Ok(Some(json!({"cmd": "read_from_disk", "mode": "ok", "msg": "", "db_key": "test"}))));
    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "append_list", "db_key": "test", "key": "list0", "val": json!(1)})).unwrap()),
                Ok(Some(json!({"cmd": "append_list", "mode": "ok", "msg": "", "db_key": "test", "key": "list0", "index": 7}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "append_list", "db_key": "test", "key": "list0", "val": json!(5)})).unwrap()),
                Ok(Some(json!({"cmd": "append_list", "mode": "ok", "msg": "", "db_key": "test", "key": "list0", "index": 8}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "append_list", "db_key": "test", "key": "list0", "val": json!(true)})).unwrap()),
                Ok(Some(json!({"cmd": "append_list", "mode": "ok", "msg": "", "db_key": "test", "key": "list0", "index": 9}))));
}

#[test]
pub fn test_get_length()
{
    let mut opt = args::Arguments::from_args();
    opt.path = std::path::PathBuf::from("test-databases");

    let server = server::Server::new(&opt).unwrap();
    let mut conn = server::ServerInterface::new(&std::sync::Arc::new(server));
    conn.fake_auth();

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "read_from_disk", "db_key": "test"})).unwrap()), 
                Ok(Some(json!({"cmd": "read_from_disk", "mode": "ok", "msg": "", "db_key": "test"}))));
    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_list_length", "db_key": "test", "key": "list0"})).unwrap()),
                Ok(Some(json!({"cmd": "get_list_length", "mode": "ok", "msg": "", "db_key": "test", "key": "list0", "length": 7}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_list_length", "db_key": "test", "key": "list1"})).unwrap()),
                Ok(Some(json!({"cmd": "get_list_length", "mode": "ok", "msg": "", "db_key": "test", "key": "list1", "length": 0}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_list_length", "db_key": "test", "key": "list2"})).unwrap()),
                Ok(Some(json!({"cmd": "get_list_length", "mode": "ok", "msg": "", "db_key": "test", "key": "list2", "length": 3}))));
}

#[test]
pub fn test_get_recent()
{
    let mut opt = args::Arguments::from_args();
    opt.path = std::path::PathBuf::from("test-databases");

    let server = server::Server::new(&opt).unwrap();
    let mut conn = server::ServerInterface::new(&std::sync::Arc::new(server));
    conn.fake_auth();

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "read_from_disk", "db_key": "test"})).unwrap()), 
                Ok(Some(json!({"cmd": "read_from_disk", "mode": "ok", "msg": "", "db_key": "test"}))));
    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_recent", "db_key": "test", "key": "list0", "num": 3})).unwrap()),
                Ok(Some(json!({"cmd": "get_recent", "mode": "ok", "msg": "", "db_key": "test", "key": "list0", "val": json!([false, 4.0, true])}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_recent", "db_key": "test", "key": "list0", "num": 5})).unwrap()),
                Ok(Some(json!({"cmd": "get_recent", "mode": "ok", "msg": "", "db_key": "test", "key": "list0", "val": json!([false, 4.0, true, "3", "2"])}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "get_recent", "db_key": "test", "key": "list0", "num": 10})).unwrap()),
                Ok(Some(json!({"cmd": "get_recent", "mode": "ok", "msg": "", "db_key": "test", "key": "list0", "val": json!([false, 4.0, true, "3", "2", 1, 0])}))));
}

#[test]
pub fn test_create_database()
{
    let mut opt = args::Arguments::from_args();
    opt.path = std::path::PathBuf::from("test-databases");

    let server = server::Server::new(&opt).unwrap();
    let mut conn = server::ServerInterface::new(&std::sync::Arc::new(server));
    conn.fake_auth();

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "create_database", "db_key": "database0"})).unwrap()),
                Ok(Some(json!({"cmd": "create_database", "mode": "ok", "msg": "", "db_key": "database0"}))));
    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "create_database", "db_key": "database1"})).unwrap()),
                Ok(Some(json!({"cmd": "create_database", "mode": "ok", "msg": "", "db_key": "database1"}))));
    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmd": "create_database", "db_key": "database2"})).unwrap()),
                Ok(Some(json!({"cmd": "create_database", "mode": "ok", "msg": "", "db_key": "database2"}))));
}