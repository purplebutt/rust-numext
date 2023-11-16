use numext::NumExt;

#[test]
fn len_int() {
    let number1 = 12345;
    let number2 = 1;
    let number3 = 0;

    let len1 = number1.len();
    let len2 = number2.len();
    let len3 = number3.len();
    
    assert_eq!(len1, 5);
    assert_eq!(len2, 1);
    assert_eq!(len3, 0);
}
#[test]
fn len_float() {
    let number1 = 12345.33;
    let number2 = 1.;
    let number3 = 0.;

    let len1 = number1.len();
    let len2 = number2.len();
    let len3 = number3.len();
    
    assert_eq!(len1, 5);
    assert_eq!(len2, 1);
    assert_eq!(len3, 0);
}
#[test]
fn pop_int() {
    let number = 12345;

    let popped1 = number.pop(1 as u32);
    let popped2 = number.pop(2u32);
    let popped3 = number.pop(3u32);
    let popped4 = number.pop(100u32);

    assert_eq!(5, popped1);
    assert_eq!(45, popped2);
    assert_eq!(345, popped3);
    assert_eq!(number, popped4);

    let zero = 0;
    let popzero = zero.pop(1u32);
    assert_eq!(0, popzero);
}
#[test]
fn pop_float() {
    let number = 12345.30;

    let popped1 = number.pop(1.);
    let popped2 = number.pop(2.);
    let popped3 = number.pop(3.);
    let popped4 = number.pop(100.);

    assert_eq!(5.299999999999272, popped1);
    assert_eq!(45.299999999999272, popped2);
    assert_eq!(345.299999999999272, popped3);
    assert_eq!(popped4, number);

    let zero = 0.0;
    let popzero = zero.pop(1.);
    assert_eq!(0.0, popzero);
}
#[test]
fn head() {
    let number = 12345;

    let head1 = number.head(1u32);
    let head2 = number.head(2u32);
    let head3 = number.head(3u32);

    assert_eq!(1, head1);
    assert_eq!(12, head2);
    assert_eq!(123, head3);

    let zero = 0;
    let headzero = zero.head(1u32);
    assert_eq!(0, headzero);
}
#[test]
fn head_float() {
    let number = 0.12_345f32;

    let head = number.head(3.);
    let len = number.len();

    assert_eq!(123., head);
    assert_eq!(0, len);

    let number = 1.12_345f32;
    let len = number.len();
    assert_eq!(1, len);

    let zero = 0.0;
    let headzero = zero.head(1.);
    assert_eq!(0., headzero);
}
#[test]
fn mid_int() {
    let number = 12345;

    let mid1 = number.mid(0u32, 2u32);
    let mid2 = number.mid(1u32, 3u32);
    let mid3 = number.mid(4u32, 1u32);
    let mid4 = number.mid(4u32, 10u32);
    let mid5 = number.mid(5u32, 1u32);

    assert_eq!(mid1, 12);
    assert_eq!(mid2, 234);
    assert_eq!(mid3, 5);
    assert_eq!(mid4, 5);
    assert_eq!(mid5, 0);
}
#[test]
fn mid_float() {
    let number = 12345.33;

    let mid1 = number.mid(0f64, 2f64);
    let mid2 = number.mid(1f64, 3f64);
    let mid3 = number.mid(4f64, 1f64);
    let mid4 = number.mid(4f64, 10f64);
    let mid5 = number.mid(5f64, 1f64);

    assert_eq!(mid1, 12.);
    assert_eq!(mid2, 234.);
    assert_eq!(mid3, 5.);
    assert_eq!(mid4, 5.);
    assert_eq!(mid5, 0.);
}
#[test]
fn float_lt_1() {
    let number = 0.12345;

    let len = number.len();
    let pop1 = number.pop(0f64);
    let pop2 = number.pop(1f64);
    let head = number.head(0f64);
    let head1 = number.head(1f64);
    let head2 = number.head(2f64);
    let mid2 = number.mid(0f64, 3f64);
    let mid = number.mid(1f64, 1f64);

    assert_eq!(len, 0);
    assert_eq!(pop1, 0.);
    assert_eq!(pop2, 0.);
    assert_eq!(head, 0.);
    assert_eq!(head1, 1.);
    assert_eq!(head2, 12.);
    assert_eq!(mid2, 12.);
    assert_eq!(mid, 1.);
}
#[test]
fn tovec_int() {
    let num1 = 12_345;
    let num2 = 123_456;
    let num3 = 0;


    let mut vec1 = num1.tovec(3u32);
    let mut vec2 = num2.tovec(3u32);
    let vec3 = num3.tovec(3u32);

    // num1
    assert_eq!(2, vec1.len());
    assert_eq!(12, vec1.pop().unwrap());
    assert_eq!(345, vec1.pop().unwrap());

    // num2
    assert_eq!(2, vec2.len());
    assert_eq!(123, vec2.pop().unwrap());
    assert_eq!(456, vec2.pop().unwrap());

    // num3
    println!("{:?}", vec3);
    println!("len: {}", num3.len());
    println!("len: {}", 0.0.len());
    assert_eq!(0, vec3.len());
}
#[test]
fn format_int() {
    let i = 12_345;
    let f = 12_345.33; 

    let fi = i.format(",");
    let ff = f.format(",");

    assert_eq!("12,345", fi);
    assert_eq!("12,345", ff);
}

