#![allow(unused)]
use std::ops::Rem;
use std::ops::Mul;


pub fn pop<T>(value: T, base: T) -> T 
where T: Rem<Output = T> + Mul<Output = T>
{
    let last = value % base;
    last
}

// pub fn lenwhile(number: i32, base: i32) -> usize {
//     let mut current = <i32>::default();
//     let mut result = 0;
//     while current != number {
//         result += 1;
//         let nbase = base.pow(result);
//         current = number % nbase;
//     }
//     result as usize
// }

// fn len_recursive_i32(number: i32, base: i32, result: u32) -> usize {
//     let mut nresult = result+1;
//     let nbase = base.pow(nresult);
//     if (number%nbase) != number {
//         nresult = len_recursive_i32(number, base, nresult) as u32;
//     }
//     nresult as usize
// }

//=============Integer==============
macro_rules! len_recursive_int {
    ($name:ident as $type:ty) => {
        pub fn $name(number: $type, base: $type, mut result: u32) -> usize {
            if number > 0 {
                let mut nresult = result + 1;
                let nbase = base.pow(nresult);
                if (number%nbase) != number {
                    nresult = $name(number, base, nresult) as u32;
                }
                nresult as usize
            } else { 0 }
        }
    };
    ($($name:ident as $type:ty),+$(,)?) => {
        $(len_recursive_int!($name as $type);)+
    }
}

len_recursive_int!(
    len_i8 as i8, len_i16 as i16, len_i32 as i32, len_i64 as i64, len_i128 as i128,
    len_u8 as u8, len_u16 as u16, len_u32 as u32, len_u64 as u64, len_u128 as u128,
);

//===============Float================
macro_rules! len_recursive_float {
    ($name:ident as $type:ty) => {
        pub fn $name(number: $type, base: $type, result: $type) -> usize {
            if number < (1 as $type) { return 0 }
            let mut nresult = result + (1 as $type);
            let nbase = base.powf(nresult);
            if (number%nbase) != number {
                nresult = $name(number, base, nresult) as $type;
            }
            nresult as usize
        }
    };
    ($($name:ident as $type:ty),+$(,)?) => {
        $(len_recursive_float!($name as $type);)+
    }
}

len_recursive_float!(len_f32 as f32, len_f64 as f64);

