#![allow(dead_code)]

use aci_server::*;

use structopt::StructOpt;

fn main()
{
    let opt = args::Arguments::from_args();
    logging::initialize_logging(&opt);

    let db = database::DatabaseInterface::new(database::Database::new("test"), chashmap::CHashMap::new());
    let user = database::UserAuthentication{is_authed: true, name: "user".to_string(), domain:"a_auth".to_string()};

    db.write_to_key("test_key", serde_json::json!("Hello World!"), &user).unwrap();
}
