use numan::helper::pop as hpop;

#[test]
fn pop() {
    const BASE: i32 = 10;
    let number = 12345;

    let popped1 = hpop(number, BASE.pow(1));
    let popped2 = hpop(number, BASE.pow(2));
    let popped3 = hpop(number, BASE.pow(3));

    assert_eq!(5, popped1);
    assert_eq!(45, popped2);
    assert_eq!(345, popped3);
}
