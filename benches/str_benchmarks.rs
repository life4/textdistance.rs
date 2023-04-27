use criterion::BenchmarkId;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use std::time::Duration;
use textdistance::textdistance::str::{damerau_levenshtein, hamming, levenshtein};

fn read_licenses() -> Vec<(String, String)> {
    let mut licenses: Vec<(String, String)> = Vec::new();
    let dir = fs::read_dir("choosealicense.com/_licenses").unwrap();
    let mut i = 0;
    for lfile in dir {
        let lpath = lfile.unwrap();
        let ltext = fs::read_to_string(lpath.path()).unwrap();
        let lname = lpath.file_name().to_str().unwrap().to_owned();
        licenses.push((lname, ltext));

        i += 1;
        if i == 5 {
            break;
        }
    }
    licenses
}

type AlgFn = dyn Fn(&str, &str) -> usize;

fn criterion_benchmark(c: &mut Criterion) {
    let licenses = read_licenses();
    let mut group = c.benchmark_group("str");
    group.sample_size(10);
    group.measurement_time(Duration::new(3, 0));
    group.warm_up_time(Duration::new(1, 0));
    // group.sampling_mode(criterion::SamplingMode::Flat);

    let algs: Vec<(&str, Box<AlgFn>)> = vec![
        ("hamming", Box::new(hamming)),
        ("levenshtein", Box::new(levenshtein)),
        ("damerau_levenshtein", Box::new(damerau_levenshtein)),
    ];

    for (alg_name, alg_fn) in algs {
        group.bench_with_input(
            BenchmarkId::from_parameter(alg_name),
            &licenses,
            |b, licenses| {
                b.iter(|| {
                    for (_, l1) in licenses {
                        for (_, l2) in licenses {
                            let s1 = black_box(l1);
                            let s2 = black_box(l2);
                            alg_fn(s1, s2);
                        }
                    }
                });
            },
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
