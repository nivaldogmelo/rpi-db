use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rpi_db::{
    domain::DatabaseTrait,
    storage::{BincodeDB, JsonDB},
};

fn bincode_insert_benchmark(c: &mut Criterion) {
    c.bench_function("bincode insert key flush on end", |b| {
	let mut db = BincodeDB::new("./bench.bin").unwrap();
	b.iter(|| {
	    for i in 0..2000 {
		let _ = black_box(db.insert(black_box(i.to_string()), black_box(i.to_string())));
	    }
	    black_box(db.flush());
	})
    });
}

fn bincode_insert_flush_benchmark(c: &mut Criterion) {
    c.bench_function("bincode insert key flush every save", |b| {
	let mut db = BincodeDB::new("./bench_flush.bin").unwrap();
	b.iter(|| {
	    for i in 0..2000 {
		let _ = black_box(db.insert(black_box(i.to_string()), black_box(i.to_string())));
		black_box(db.flush());
	    }
	})
    });
}

fn bincode_get_benchmark(c: &mut Criterion) {
    c.bench_function("bincode get key", |b| {
	let db = BincodeDB::new("./bench.bin").unwrap();
	b.iter(|| {
	    for i in 0..2000 {
		let _ = black_box(db.search(black_box(i.to_string())));
	    }
	})
    });
}

fn json_insert_benchmark(c: &mut Criterion) {
    c.bench_function("json insert key ", |b| {
	let mut db = JsonDB::new("./bench.json").unwrap();
	b.iter(|| {
	    for i in 0..2000 {
		let _ = db.insert(black_box(i.to_string()), black_box(i.to_string()));
	    }
	    black_box(db.flush());
	})
    });
}

fn json_insert_flush_benchmark(c: &mut Criterion) {
    c.bench_function("json insert key flush every save", |b| {
	let mut db = JsonDB::new("./bench_flush.json").unwrap();
	b.iter(|| {
	    for i in 0..2000 {
		let _ = db.insert(black_box(i.to_string()), black_box(i.to_string()));
		black_box(db.flush());
	    }
	})
    });
}

fn json_get_benchmark(c: &mut Criterion) {
    c.bench_function("json get key", |b| {
	let db = JsonDB::new("./bench.json").unwrap();
	b.iter(|| {
	    for i in 0..2000 {
		let _ = black_box(db.search(black_box(i.to_string())));
	    }
	})
    });
}

criterion_group!(
    benches,
    json_insert_benchmark,
    json_get_benchmark,
    json_insert_flush_benchmark,
    bincode_insert_benchmark,
    bincode_get_benchmark,
    bincode_insert_flush_benchmark,
);
criterion_main!(benches);
