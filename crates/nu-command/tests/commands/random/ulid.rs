use nu_test_support::nu;

#[test]
fn generates_valid_ulid() {
    let actual = nu!("random ulid");

    let result = ulid::parse_str(actual.out.as_str());

    assert!(result.is_ok());
}
