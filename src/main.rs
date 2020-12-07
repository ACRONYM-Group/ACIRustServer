#![allow(dead_code)]

use aci_server::*;

use structopt::StructOpt;

fn main()
{
    let opt = args::Arguments::from_args();
    logging::initialize_logging(&opt);

    let db = database::database_from_disk(&opt.path, "command", &opt).unwrap();
    let user = database::UserAuthentication{is_authed: true, name: "user".to_string(), domain:"a_auth".to_string()};

    println!("{:?}", db.read_from_key("test_begin", &user));
    println!("{:?}", db.read_from_key("test_end", &user));
}
