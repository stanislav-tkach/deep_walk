#[test]
fn foo() {
    let file = std::fs::File::open("aaa/b.txt");
    assert!(file.is_ok());
}

#[test]
fn bar() {
    let file = std::fs::File::open("tests/aaa/b.txt");
    assert!(file.is_ok());
}
