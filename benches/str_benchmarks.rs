use core::fs;
use core::time::Duration;
use criterion::BenchmarkId;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use textdistance::{nstr, str};

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

type AlgFn = dyn Fn(&str, &str) -> f64;

fn criterion_benchmark(c: &mut Criterion) {
    benchmark_nstr(c);
}

fn benchmark_nstr(c: &mut Criterion) {
    let licenses = read_licenses();
    let mut group = c.benchmark_group("nstr");
    group.sample_size(10);
    group.measurement_time(Duration::new(3, 0));
    group.warm_up_time(Duration::new(1, 0));
    // group.sampling_mode(criterion::SamplingMode::Flat);

    let algs: Vec<(&str, Box<AlgFn>)> = vec![
        ("bag", Box::new(nstr::bag)),
        ("cosine", Box::new(nstr::cosine)),
        ("damerau_levenshtein", Box::new(nstr::damerau_levenshtein)),
        (
            "damerau_levenshtein_restricted",
            Box::new(nstr::damerau_levenshtein_restricted),
        ),
        ("entropy_ncd", Box::new(nstr::entropy_ncd)),
        ("hamming", Box::new(nstr::hamming)),
        ("jaccard", Box::new(nstr::jaccard)),
        ("jaro_winkler", Box::new(nstr::jaro_winkler)),
        ("jaro", Box::new(nstr::jaro)),
        ("lcsseq", Box::new(nstr::lcsseq)),
        ("lcsstr", Box::new(nstr::lcsstr)),
        ("length", Box::new(nstr::length)),
        ("levenshtein", Box::new(nstr::levenshtein)),
        ("lig3", Box::new(nstr::lig3)),
        ("mlipns", Box::new(nstr::mlipns)),
        ("overlap", Box::new(nstr::overlap)),
        ("prefix", Box::new(nstr::prefix)),
        ("ratcliff_obershelp", Box::new(nstr::ratcliff_obershelp)),
        ("roberts", Box::new(nstr::roberts)),
        ("sift4_common", Box::new(nstr::sift4_common)),
        ("sift4_simple", Box::new(nstr::sift4_simple)),
        ("smith_waterman", Box::new(nstr::smith_waterman)),
        ("sorensen_dice", Box::new(nstr::sorensen_dice)),
        ("suffix", Box::new(nstr::suffix)),
        ("tversky", Box::new(nstr::tversky)),
        ("yujian_bo", Box::new(nstr::yujian_bo)),
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
