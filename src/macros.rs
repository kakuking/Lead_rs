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
macro_rules! impl_operator_2 {
    ($struct:ident<$generic:ident>, $trait:ident, $func:ident, $op:tt) => {
        impl<$generic> $trait for $struct<$generic>
        where
            $generic: $trait<Output = $generic> + Copy + Float,
        {
            type Output = Self;

            fn $func(self, rhs: Self) -> Self::Output {
                Self {
                    x: self.x $op rhs.x,
                    y: self.y $op rhs.y,
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_operator_unary_2 {
    ($struct:ident<$generic:ident>, $trait:ident, $func:ident, $op:tt) => {
        impl<$generic> $trait for $struct<$generic>
        where
            $generic: $trait<Output = $generic> + Copy + Float,
        {
            type Output = Self;

            fn $func(self) -> Self::Output {
                Self {
                    x: $op self.x,
                    y: $op self.y,
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_operator_inplace_2 {
    ($struct:ident<$generic:ident>, $trait:ident, $func:ident, $op:tt) => {
        impl<$generic> $trait for $struct<$generic>
        where
            $generic: std::ops::$trait + Copy + Float,
        {
            fn $func(&mut self, rhs: Self) {
                self.x $op rhs.x;
                self.y $op rhs.y;
            }
        }
    };
}

#[macro_export]
macro_rules! impl_operator_3 {
    ($struct:ident<$generic:ident>, $trait:ident, $func:ident, $op:tt) => {
        impl<$generic> $trait for $struct<$generic>
        where
            $generic: $trait<Output = $generic> + Copy + Float,
        {
            type Output = Self;

            fn $func(self, rhs: Self) -> Self::Output {
                Self {
                    x: self.x $op rhs.x,
                    y: self.y $op rhs.y,
                    z: self.z $op rhs.z,
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_operator_unary_3 {
    ($struct:ident<$generic:ident>, $trait:ident, $func:ident, $op:tt) => {
        impl<$generic> $trait for $struct<$generic>
        where
            $generic: $trait<Output = $generic> + Copy + Float,
        {
            type Output = Self;

            fn $func(self) -> Self::Output {
                Self {
                    x: $op self.x,
                    y: $op self.y,
                    z: $op self.z,
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_operator_inplace_3 {
    ($struct:ident<$generic:ident>, $trait:ident, $func:ident, $op:tt) => {
        impl<$generic> $trait for $struct<$generic>
        where
            $generic: std::ops::$trait + Copy + Float,
        {
            fn $func(&mut self, rhs: Self) {
                self.x $op rhs.x;
                self.y $op rhs.y;
                self.z $op rhs.z;
            }
        }
    };
}


pub use register_struct;
pub use impl_operator_2;
pub use impl_operator_3;
pub use impl_operator_unary_2;
pub use impl_operator_unary_3;
pub use impl_operator_inplace_2;
pub use impl_operator_inplace_3;