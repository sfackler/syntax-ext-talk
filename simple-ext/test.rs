#[feature(phase)];

#[phase(syntax)]
extern crate sort;

#[test]
fn test_sort() {
    let arr = sort!("z", "hello", "a", "");
    let expected = ["", "a", "hello", "z"];

    assert_eq!(expected, arr);
}
