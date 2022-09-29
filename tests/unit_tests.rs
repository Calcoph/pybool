use pybool::pybool;

#[test]
fn it_works() {
    assert_eq!(pybool!(true or true), true);
    assert_eq!(pybool!(true or false), true);
    assert_eq!(pybool!(false or true), true);
    assert_eq!(pybool!(false or false), false);

    assert_eq!(pybool!(true and true), true);
    assert_eq!(pybool!(true and false), false);
    assert_eq!(pybool!(false and true), false);
    assert_eq!(pybool!(false and false), false);

    assert_eq!(pybool!(not true), false);
    assert_eq!(pybool!(not false), true);

    assert_eq!(pybool!((false or not (false or true)) and not false), false);
}