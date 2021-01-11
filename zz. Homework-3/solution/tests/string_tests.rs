use solution::*;

#[test]
fn skip_next_test() {
    assert_eq!(skip_next("(foo", '('), Some("foo"));
    assert_eq!(skip_next("(foo", ')'), None);
    assert_eq!(skip_next("", '('), None);
}

#[test]
fn take_until_test() {
    assert_eq!(take_until(" foo/bar ", '/'), (" foo", "/bar "));
    assert_eq!(take_until("foobar", '/'), ("foobar", ""));
    assert_eq!(take_until("foobar/", '/'), ("foobar", "/"));
    assert_eq!(take_until("/foobar", '/'), ("", "/foobar"));
    assert_eq!(take_until("", '/'), ("", ""));
}

#[test]
fn take_and_skip_test() {
    assert_eq!(take_and_skip(" foo/bar ", '/'), Some((" foo", "bar ")));
    assert_eq!(take_and_skip("foobar", '/'), None);
    assert_eq!(take_and_skip("foobar/", '/'), Some(("foobar", "")));
    assert_eq!(take_and_skip("/foobar", '/'), Some(("", "foobar")));
    assert_eq!(take_and_skip("fooчbar", 'ч'), Some(("foo", "bar")));
}
