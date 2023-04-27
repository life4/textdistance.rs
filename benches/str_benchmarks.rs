use criterion::BenchmarkId;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use std::time::Duration;
use textdistance::textdistance::str::hamming;

fn criterion_benchmark(c: &mut Criterion) {
    let mut licenses: Vec<(String, String)> = Vec::new();
    let dir = fs::read_dir("choosealicense.com/_licenses").unwrap();
    for lfile in dir {
        let lpath = lfile.unwrap();
        let ltext = fs::read_to_string(lpath.path()).unwrap();
        let lname = lpath.file_name().to_str().unwrap().to_owned();
        licenses.push((lname, ltext));
    }

    let mut group = c.benchmark_group("hamming");
    group.sample_size(10);
    group.measurement_time(Duration::new(1, 0));
    for (lname, ltext) in licenses {
        group.bench_with_input(BenchmarkId::from_parameter(&lname), &ltext, |b, license| {
            b.iter(|| {
                hamming(black_box(license), black_box(license));
            });
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
