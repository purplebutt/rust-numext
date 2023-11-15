#![allow(dead_code)]
use numan::{Numan, Len};

// fn head(value: i32, n: u32) -> i32 {
//     let len = value.len();
//     let x = 10i32.pow(len as u32 - n);
//     value / x
// }

fn main() {
    let number: u32 = 10284;
    
    let popped = number.pop(4);
    let headed = number.head(3);
    println!("ori:\t\t{number}");
    println!("pop(4)->\t{popped}");
    println!("head(3)->\t{headed}");
    
    let len = number.len();

    println!("{len}");
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

