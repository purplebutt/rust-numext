
pub trait Numan<T> {
    /// for signed and unsigned integer:
    /// this function call pow() of integer
    /// since pow() always required a u32 type for it's
    /// argument, then pop() should also provided with u32
    fn pop(&self, n: T) -> Self;
}

macro_rules! Impl_Numan_float {
    ($target:ty) => {
        impl Numan<$target> for $target {
            fn pop(&self, n: $target) -> Self {
                let b = 10 as $target;
                let base = b.powf(n);
                $crate::helper::pop(*self, base)
            }
        }
    };
    ($($type:ty),+$(,)?) => {
        $(Impl_Numan_float!($type);)+
    }
}

#[macro_export]
macro_rules! Impl_Numan_int {
    ($target:ty) => {
        impl Numan<u32> for $target {
            /// this function call pow() of stringify!($target)
            /// since pow() always required a u32 type for it's
            /// argument, then pop() should also provided with u32
            fn pop(&self, n: u32) -> Self {
                let b = 10 as $target;
                let base = b.pow(n);
                $crate::helper::pop(*self, base)
            }
        }
    };
    ($($type:ty),+$(,)?) => {
        $(Impl_Numan_int!($type);)+
    };
}

Impl_Numan_int!(i8,i16,i32,i64,i128,u8,u16,u32,u64,u128,);
Impl_Numan_float!(f32,f64,);

