#![allow(dead_code)]

mod args;
mod logging;
mod commands;
mod database;

use structopt::StructOpt;

static BUILD_VERSION: &str = "dev2020.12.05.1";

fn main()
{
    let opt = args::Arguments::from_args();
    logging::initialize_logging(&opt);

    let db = database::DatabaseInterface::new(database::Database::new("Database0"), chashmap::CHashMap::new());
    let user = database::UserAuthentication{is_authed: true, name: "user".to_string(), domain:"a_auth".to_string()};

    db.write_to_key("key", serde_json::json!(0), &user).unwrap();
    println!("{:?}", db.read_from_key("key", &user));

    db.write_to_key("list", serde_json::json!([]), &user).unwrap();
    println!("{:?}", db.read_from_key("key", &user));
    println!("{:?}", db.get_length_from_key("key", &user));
}
