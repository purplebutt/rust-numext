#![allow(dead_code)]
use numext::NumExt;

fn main() {
    demo1();
    demo2();

    let int = 12345270;
    let flt = 12740.34;

    println!("{}", int.format(","));
    println!("{:?}", flt.tovec(3_f64));
    println!("{}", flt.format(","));
}

fn demo1() {
    let n = 12345;

    let pop1 = n.pop(1u32);
    let pop2 = n.pop(2u32);
    let pop3 = n.pop(3u32);

    println!("{} {} {}", pop1, pop2, pop3);

    let m = 12345.0;
    let pop1 = m.pop(1.);
    let pop2 = m.pop(2.);
    let pop3 = m.pop(3.);

    println!("{} {} {}", pop1, pop2, pop3);

    let m = 12345_f32;
    let pop1 = m.pop(1.);
    let pop2 = m.pop(2.);
    let pop3 = m.pop(3.);

    println!("{} {} {}", pop1, pop2, pop3);
}

fn demo2() {
    let number: u32 = 170284;
    
    let popped = number.pop(5);
    let headed = number.head(3);
    let mid1_3 = number.mid(1,3);
    let mid3_2 = number.mid(3,2);
    let mid3_20 = number.mid(3,20);
    println!("ori:\t\t{number}");
    println!("pop(4)->\t{popped}");
    println!("head(3)->\t{headed}");
    println!("mid(1,3)->\t{mid1_3}");
    println!("mid(3,2)->\t{mid3_2}");
    println!("mid(3,20)->\t{mid3_20}");
    
    let len = number.len();

    println!("{len}");
}

