mod rectangle_test;

#[test]
fn exploration() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[ignore]
#[test]
fn another_panic_test() {
    panic!("Test failed with panic!")
}

#[test]
fn should_return_true_when_rect1_is_bigger_than_rect2() {
    //AAA
    let rect1 = rectangle_test::Rectangle::new(5, 11);
    let rect2 = rectangle_test::Rectangle::new(3, 4);
    assert_eq!(true, rect1.can_hold(rect2))
}



