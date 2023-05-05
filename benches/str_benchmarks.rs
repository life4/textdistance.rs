use criterion::BenchmarkId;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use std::time::Duration;
use textdistance::str;

fn read_licenses() -> Vec<(String, String)> {
    let mut licenses: Vec<(String, String)> = Vec::new();
    let dir = fs::read_dir("choosealicense.com/_licenses").unwrap();
    let mut i = 0;
    for lfile in dir {
        let lpath = lfile.unwrap();
        let ltext = fs::read_to_string(lpath.path()).unwrap();
        let lname = lpath.file_name().to_str().unwrap().to_owned();
        // shorten the text to speed up benchmarks run
        let ltext = ltext[1..200].to_string();
        licenses.push((lname, ltext));

        // take only a subset of licenses to speed up benchmarks run
        i += 1;
        if i == 10 {
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
        ("damerau_levenshtein", Box::new(str::damerau_levenshtein)),
        (
            "damerau_levenshtein_restricted",
            Box::new(str::damerau_levenshtein_restricted),
        ),
        ("hamming", Box::new(str::hamming)),
        ("lcsseq", Box::new(str::lcsseq)),
        ("lcsstr", Box::new(str::lcsstr)),
        ("levenshtein", Box::new(str::levenshtein)),
        // ("ratcliff_obershelp", Box::new(str::ratcliff_obershelp)),
        ("sift4", Box::new(str::sift4_simple)),
        // ("jaro", Box::new(str::jaro)),
        // ("jaro_winkler", Box::new(str::jaro_winkler)),
        // ("yujian_bo", Box::new(str::yujian_bo)),
        ("mlipns", Box::new(str::mlipns)),
        // ("jaccard", Box::new(str::jaccard)),
        // ("sorensen_dice", Box::new(str::sorensen_dice)),
        // ("tversky", Box::new(str::tversky)),
        // ("overlap", Box::new(str::overlap)),
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
