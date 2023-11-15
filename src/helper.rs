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

macro_rules! len_recursive {
    ($name:ident as $type:ty) => {
        pub fn $name(number: $type, base: $type, result: u32) -> usize {
            let mut nresult = result + 1;
            let nbase = base.pow(nresult);
            if (number%nbase) != number {
                nresult = $name(number, base, nresult) as u32;
            }
            nresult as usize
        }
    };
    ($($name:ident as $type:ty),+$(,)?) => {
        $(len_recursive!($name as $type);)+
    }
}

len_recursive!(
    len_i8 as i8, 
    len_i16 as i16,
    len_i32 as i32,
    len_i64 as i64,
    len_i128 as i128,
    len_u8 as u8, 
    len_u16 as u16,
    len_u32 as u32,
    len_u64 as u64,
    len_u128 as u128,
);

// len_recursive!(len_i8 as i32);
// len_recursive!(len_i16 as i32);
// len_recursive!(len_i32 as i32);
// len_recursive!(len_i64 as i64);
// len_recursive!(len_i128 as i128);
// len_recursive!(len_u8 as u32);
// len_recursive!(len_u16 as u32);
// len_recursive!(len_u32 as u32);
// len_recursive!(len_u64 as u64);
// len_recursive!(len_u128 as u128);

