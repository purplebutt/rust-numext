use crate::helper::*;

pub trait NumExt {
    type PwrType;
    fn len(&self) -> usize;
    fn head(&self, n: Self::PwrType) -> Self;
    fn mid(&self, start: Self::PwrType, len: Self::PwrType) -> Self;
    fn pop(&self, n: Self::PwrType) -> Self;
    fn tovec(&self, by: Self::PwrType) -> Vec<Self> where Self: Sized;
    fn format(&self, sep: &str) -> String;
}

macro_rules! Impl_len_int {
    ($name:ident as $type:ty) => {
        impl NumExt for $type {
            type PwrType = u32;
            fn len(&self) -> usize { $name(*self, 10, 0) } 
            fn head(&self, n: Self::PwrType) -> Self {
                let len = self.len() as u32;
                let base = 10 as Self;
                let pwr = base.pow(len - n.clamp(0, len));
                *self / pwr
            }
            /// # panic
            /// panic if (self.len - start) < len
            fn mid(&self, start: u32, len: u32) -> Self {
                let l = self.len();
                // assert!((start as usize) < l, "Invalid argument `start`. Required `start` to be smaller than number length!");
                if (start as usize) > l { return 0 as Self }
                let popped = self.pop(l as u32 - start);
                let n = l as u32 - start;
                let pwr = 10 as Self;
                popped / (pwr.pow(n-(len.clamp(0, n))))
            }
            fn pop(&self, n: Self::PwrType) -> Self {
                let b = 10 as $type;
                let base = b.pow(n.clamp(0, self.len() as Self::PwrType));
                $crate::helper::pop(*self, base)
            }
            fn tovec(&self, by: Self::PwrType) -> Vec<Self> where Self: Sized {
                let len = self.len();
                let div = len / by as usize;
                let rem = len % by as usize;
                let mut vec = Vec::<Self>::new();
                for i in 1..=div {
                    let start = len as u32 - (by*i as u32);
                    vec.push(self.mid(start, by));
                }
                if rem > 0 { vec.push(self.head(rem as u32)); }
                vec
            }
            fn format(&self, sep: &str) -> String {
                let mut s = String::new();
                let v = self.tovec(3u32);
                for i in v.into_iter().rev() {
                    s.push_str(&i.to_string());
                    s.push_str(sep)
                }
                s.pop();
                s
            }
        }
    };
    ($($name:ident as $type:ty),+$(,)?) => {
        $(Impl_len_int!($name as $type);)+
    }
}

Impl_len_int!(
    len_i8 as i8, len_i16 as i16, len_i32 as i32, len_i64 as i64, len_i128 as i128,
    len_u8 as u8, len_u16 as u16, len_u32 as u32, len_u64 as u64, len_u128 as u128
);


macro_rules! Impl_len_float {
    ($name:ident as $type:ty) => {
        impl NumExt for $type {
            type PwrType = Self;
            fn len(&self) -> usize { $name(*self, 10 as Self, 0 as Self) } 
            fn head(&self, n: Self::PwrType) -> Self {
                if self < &(1 as Self)  {
                    let base = 10 as Self;
                    let pwr = base.powf(n);
                    (self * pwr) as usize as Self
                } else {
                    let len = self.len() as Self;
                    let base = 10 as Self;
                    let pwr = base.powf(len - n);
                    (*self / pwr) as usize as Self
                }
            }
            /// # panic
            /// panic if (self.len - start) < len
            fn mid(&self, start: Self::PwrType, len: Self::PwrType) -> Self {
                if self.len() >= 1 {
                    let l = self.len() as Self;
                    let popped = self.pop(l - start);
                    let n = l - start;
                    let pwr = 10 as Self;
                    let mxlen = len.clamp(0 as Self, n);
                    (popped / pwr.powf(n-mxlen)) as usize as Self
                }
                else {
                    let base = 10 as Self;
                    let pwr = base.powf(start+len-(1 as Self));
                    let num = *self*pwr;
                    (num % base.powf(len)) as usize as Self
                }
            }
            fn pop(&self, n: Self::PwrType) -> Self {
                if self.len() >= 1 {
                    let b = 10 as $type;
                    let base = b.powf(n);
                    $crate::helper::pop(*self, base)
                } else { 0 as Self }
            }
            fn tovec(&self, by: Self::PwrType) -> Vec<Self> where Self: Sized {
                let len = self.len() as Self::PwrType;
                let div = (len / by) as usize;
                let rem = len % by;
                let mut vec = Vec::<Self>::new();
                for i in 1..=div {
                    let start = len - (by*i as Self::PwrType);
                    vec.push(self.mid(start, by));
                }
                if rem > 0 as Self { vec.push(self.head(rem)); }
                vec
            }
            fn format(&self, sep: &str) -> String {
                let mut s = String::new();
                let v = self.tovec(3 as Self);
                for i in v.into_iter().rev() {
                    s.push_str(&i.to_string());
                    s.push_str(sep)
                }
                s.pop();
                s
            }
        }
    };
    ($($name:ident as $type:ty),+$(,)?) => {
        $(Impl_len_float!($name as $type);)+
    }
}

Impl_len_float!(len_f32 as f32, len_f64 as f64);

