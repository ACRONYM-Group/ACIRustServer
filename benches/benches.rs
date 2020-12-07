extern crate aci_server;

use criterion::{criterion_group, criterion_main, Criterion};
use std::thread;
use std::sync::Arc;
use aci_server::database::{Database, DatabaseInterface, UserAuthentication};

pub fn benchmarking(c: &mut Criterion)
{
    let db = Arc::new(DatabaseInterface::new(Database::new("Database0"), chashmap::CHashMap::new()));
    let user = UserAuthentication{is_authed: true, name: "user".to_string(), domain:"a_auth".to_string()};

    let mut group = c.benchmark_group("Single Threaded");
    group.sample_size(1000);

    group.bench_function("Writing", |b| b.iter(|| db.write_to_key("key", criterion::black_box(serde_json::json!(0)), &user).unwrap()));
    group.bench_function("Reading", |b| b.iter(|| db.read_from_key("key", &user).unwrap()));
    
    group.finish();

    let mut group = c.benchmark_group("With Writing Threads");
    group.sample_size(1000);

    let db2 = db.clone();
    let user2 = user.clone();

    let tag = thread::spawn(move || loop {db2.write_to_key("key", criterion::black_box(serde_json::json!(0)) , &user2).unwrap();});

    group.bench_function("Writing", |b| b.iter(|| db.write_to_key("key", criterion::black_box(serde_json::json!(0)), &user).unwrap()));
    group.bench_function("Reading", |b| b.iter(|| db.read_from_key("key", &user).unwrap()));
    
    group.finish();

    tag.join();
}

criterion_group!(benches, benchmarking);
criterion_main!(benches);