extern crate aci_server;

use clap_verbosity_flag::Verbosity;
use criterion::{criterion_group, criterion_main, Criterion};
use std::thread;
use std::sync::Arc;
use aci_server::*;

use structopt::StructOpt;

use serde_json::json;

pub fn benchmarking(c: &mut Criterion)
{
    let mut opt = args::Arguments::from_iter(Vec::<String>::new().into_iter());
    opt.path = std::path::PathBuf::from("test-databases");

    let server = server::Server::new(&opt);
    let arc = std::sync::Arc::new(server);
    let mut conn = server::ServerInterface::new(&arc);
    conn.fake_auth();

    let mut conn2 = server::ServerInterface::new(&arc);
    conn2.fake_auth();

    let mut conn3 = server::ServerInterface::new(&arc);
    conn3.fake_auth();

    let mut conn4 = server::ServerInterface::new(&arc);
    conn4.fake_auth();

    let mut conn5 = server::ServerInterface::new(&arc);
    conn5.fake_auth();

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmdType": "rfd", "db_key": "command"})).unwrap()), Ok(None));
    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmdType": "get_val", "db_key": "command", "key": "test_begin"})).unwrap()),
                Ok(Some(json!({"cmdType": "getResp", "key": "test_begin", "val": json!("True"), "db_key": "command"}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmdType": "get_val", "db_key": "command", "key": "test_end"})).unwrap()),
                Ok(Some(json!({"cmdType": "getResp", "key": "test_end", "val": json!("False"), "db_key": "command"}))));

    assert_eq!(conn.execute_command(commands::Command::from_json(json!({"cmdType": "get_val", "db_key": "command", "key": "load_cell_known_mass"})).unwrap()),
                Ok(Some(json!({"cmdType": "getResp", "key": "load_cell_known_mass", "val": json!(0), "db_key": "command"}))));

    let mut group = c.benchmark_group("Single Threaded");
    group.sample_size(1000);

    group.bench_function("Writing", |b| b.iter(|| conn.execute_command(commands::Command::from_json(json!({"cmdType": "set_val", "db_key": "command", "key": "test_end", "val": criterion::black_box(serde_json::json!(0))})).unwrap()).unwrap()));
    group.bench_function("Reading", |b| b.iter(|| conn.execute_command(commands::Command::from_json(json!({"cmdType": "get_val", "db_key": "command", "key": "test_end"})).unwrap()).unwrap()));
    
    group.finish();

    let mut group = c.benchmark_group("With Writing Thread");
    group.sample_size(1000);

    thread::spawn(move || loop {conn2.execute_command(commands::Command::from_json(json!({"cmdType": "set_val", "db_key": "command", "key": "test_end", "val": criterion::black_box(serde_json::json!(0))})).unwrap()).unwrap();});

    group.bench_function("Writing", |b| b.iter(|| conn.execute_command(commands::Command::from_json(json!({"cmdType": "set_val", "db_key": "command", "key": "test_end", "val": criterion::black_box(serde_json::json!(0))})).unwrap()).unwrap()));
    group.bench_function("Reading", |b| b.iter(|| conn.execute_command(commands::Command::from_json(json!({"cmdType": "get_val", "db_key": "command", "key": "test_end"})).unwrap()).unwrap()));
    
    group.finish();

    let mut group = c.benchmark_group("With 2 Writing Threads");
    group.sample_size(1000);

    thread::spawn(move || loop {conn3.execute_command(commands::Command::from_json(json!({"cmdType": "set_val", "db_key": "command", "key": "test_end", "val": criterion::black_box(serde_json::json!(0))})).unwrap()).unwrap();});

    group.bench_function("Writing", |b| b.iter(|| conn.execute_command(commands::Command::from_json(json!({"cmdType": "set_val", "db_key": "command", "key": "test_end", "val": criterion::black_box(serde_json::json!(0))})).unwrap()).unwrap()));
    group.bench_function("Reading", |b| b.iter(|| conn.execute_command(commands::Command::from_json(json!({"cmdType": "get_val", "db_key": "command", "key": "test_end"})).unwrap()).unwrap()));
    
    group.finish();

    let mut group = c.benchmark_group("With 4 Writing Threads");
    group.sample_size(1000);

    thread::spawn(move || loop {conn4.execute_command(commands::Command::from_json(json!({"cmdType": "set_val", "db_key": "command", "key": "test_end", "val": criterion::black_box(serde_json::json!(0))})).unwrap()).unwrap();});
    thread::spawn(move || loop {conn5.execute_command(commands::Command::from_json(json!({"cmdType": "set_val", "db_key": "command", "key": "test_end", "val": criterion::black_box(serde_json::json!(0))})).unwrap()).unwrap();});

    group.bench_function("Writing", |b| b.iter(|| conn.execute_command(commands::Command::from_json(json!({"cmdType": "set_val", "db_key": "command", "key": "test_end", "val": criterion::black_box(serde_json::json!(0))})).unwrap()).unwrap()));
    group.bench_function("Reading", |b| b.iter(|| conn.execute_command(commands::Command::from_json(json!({"cmdType": "get_val", "db_key": "command", "key": "test_end"})).unwrap()).unwrap()));
    
    group.finish();
}

criterion_group!(benches, benchmarking);
criterion_main!(benches);