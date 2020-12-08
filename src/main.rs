#![allow(dead_code)]

use aci_server::*;

use structopt::StructOpt;

fn main()
{
    let opt = args::Arguments::from_args();
    logging::initialize_logging(&opt);

    let server = server::Server::new(&opt);
    let mut conn = server::ServerInterface::new(&std::sync::Arc::new(server));

    conn.execute_command(commands::Command::from_json(serde_json::json!({"cmdType": "rfd", "db_key": "command"})).unwrap()).unwrap();
    println!("{:?}", conn.execute_command(commands::Command::from_json(serde_json::json!({"cmdType": "get_val", "db_key": "command", "key": "test_end"})).unwrap()).unwrap());
}
