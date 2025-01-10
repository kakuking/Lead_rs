#[macro_export]
macro_rules! register_struct {
    ($inp1:expr, $inp2:expr) => {
        #[ctor::ctor]
        fn register_sphere() {
            register_lead_object($inp1, $inp2);
        }
    };
}

#[macro_export]
macro_rules! impl_operator {
    ($struct:ident<$generic:ident, const $size:ident: usize>, $trait:ident, $func:ident, $op:tt, $opt:ident<$generic_op:ident, const $size_op:ident: usize>) => {
        impl<$generic, const $size: usize> $trait for $struct<$generic, $size>
        where
            $generic: $trait<Output = $generic> + Float + Copy + Display + FromStr,
            <T as FromStr>::Err: std::fmt::Debug 
        {
            type Output = $opt<$generic_op, $size>;

            fn $func(self, rhs: Self) -> Self::Output {
                let mut result = [self[0] $op rhs[0]; $size];
                for i in 0..$size {
                    result[i] = self[i] $op rhs[i];
                }
                $opt::<$generic_op, $size>::init(result)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_operator_unary {
    ($struct:ident<$generic:ident, const $size:ident: usize>, $trait:ident, $func:ident, $op:tt) => {
        impl<$generic, const $size: usize> $trait for $struct<$generic, $size>
        where
            $generic: $trait<Output = $generic> + Float + Copy + Display + FromStr,
            <T as FromStr>::Err: std::fmt::Debug 
        {
            type Output = Self;

            fn $func(self) -> Self::Output {
                let mut result = [self[0]; $size];
                for i in 0..$size {
                    result[i] = $op self[i];
                }
                Self::init(result)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_operator_inplace {
    ($struct:ident<$generic:ident, const $size:ident: usize>, $trait:ident, $func:ident, $op:tt) => {
        impl<$generic, const $size: usize> $trait for $struct<$generic, $size>
        where
            $generic: std::ops::$trait + Float + Copy + Display + FromStr,
            <T as FromStr>::Err: std::fmt::Debug 
        {
            fn $func(&mut self, rhs: Self) {
                for i in 0..$size {
                    self[i] $op rhs[i];
                }
            }
        }
    };
}


pub use register_struct;
pub use impl_operator;
pub use impl_operator_unary;
pub use impl_operator_inplace;