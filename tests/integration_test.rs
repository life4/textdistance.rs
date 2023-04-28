#[test]
fn test_str_hamming() {
    let res = textdistance::str::hamming("hello", "hi");
    assert_eq!(res, 4);
}
