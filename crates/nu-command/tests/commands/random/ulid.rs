use nu_test_support::nu;
use ulid::Ulid;

#[test]
fn generates_valid_ulid() {
    let actual = nu!("random ulid");
    let result = Ulid::from_string(actual.out.as_str());

    assert_eq!(result.is_ok(), true);
}
