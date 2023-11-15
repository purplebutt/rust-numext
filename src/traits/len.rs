use crate::helper::*;

pub trait Len {
    fn len(&self) -> usize;
    fn head(&self, n: u32) -> Self;
}

macro_rules! Impl_len {
    ($name:ident as $type:ty) => {
        impl Len for $type {
            fn len(&self) -> usize { $name(*self, 10, 0) } 
            fn head(&self, n: u32) -> Self {
                let len = self.len();
                let base = 10 as Self;
                let pwr = base.pow(len as u32 - n);
                *self / pwr
            }
        }
    };
    ($($name:ident as $type:ty),+$(,)?) => {
        $(Impl_len!($name as $type);)+
    }
}

Impl_len!(
    len_i8 as i8, len_i16 as i16, len_i32 as i32, len_i64 as i64, len_i128 as i128,
    len_u8 as u8, len_u16 as u16, len_u32 as u32, len_u64 as u64, len_u128 as u128
);


