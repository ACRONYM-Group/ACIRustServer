#![allow(dead_code)]

use aci_server::*;

use structopt::StructOpt;

fn main()
{
    let opt = args::Arguments::from_args();
    logging::initialize_logging(&opt);

    let user = database::UserAuthentication{is_authed: true, name: "user".to_string(), domain:"a_auth".to_string()};

    // Write to test database
    {
        let db = database::DatabaseInterface::new(database::Database::new("test"), chashmap::CHashMap::new());

        db.write_to_key("test_key", serde_json::json!("Hello World!"), &user).unwrap();

        database::database_to_disk(&opt.path, db, &opt).unwrap();
    }

    // Read from test database
    {
        let db = database::database_from_disk(&opt.path, "test", &opt).unwrap();

        println!("Value: {:?}", db.read_from_key("test_key", &user));
    }
}
