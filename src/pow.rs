pub trait Pow<T> {
    fn powt(&self, exponent: T) -> Self;
}

macro_rules! impl_pow {
    ($($t:ty),+) => {
        $(
            impl Pow<$t> for $t {
                fn powt(&self, exponent: $t) -> $t {
                    self.pow(exponent as u32)
                }
            }
        ),*
    };
}

impl_pow!(i32);
