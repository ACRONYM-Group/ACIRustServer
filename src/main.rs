#![allow(dead_code)]

use aci_server::*;

use structopt::StructOpt;

fn main()
{
    let opt = args::Arguments::from_args();
    logging::initialize_logging(&opt);

    let db = database::Database::new("test");
    let dbi = database::DatabaseInterface::new(db, chashmap::CHashMap::new());

    let mut user = database::UserAuthentication::new();
    user.is_authed = true;

    dbi.write_to_key("list0", serde_json::json!([0, 1, "2", "3", true, 4.0, false]), &user).unwrap();
    dbi.write_to_key("list1", serde_json::json!([]), &user).unwrap();
    dbi.write_to_key("list2", serde_json::json!([0, 1, false]), &user).unwrap();

    database::database_to_disk(&opt.path.clone(), dbi, &opt).unwrap();
}
