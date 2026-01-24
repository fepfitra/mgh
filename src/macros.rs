macro_rules! impl_static {
    ($($name:ident),*) => {
        impl MGHInit {
            $(
                pub fn $name() -> Vec<f64> {
                    problems::$name::init()
                }
            )*
        }
        impl MGH {
            $(
                pub fn $name(x: &[f64]) -> f64 {
                    problems::$name::$name(x)
                }
            )*
        }
    }
}

macro_rules! impl_n {
    ($($name:ident),*) => {
        impl MGHInit {
            $(
                pub fn $name(n: usize) -> Vec<f64> {
                    problems::$name::init(n)
                }
            )*
        }
        impl MGH {
            $(
                pub fn $name(x: &[f64]) -> f64 {
                    problems::$name::$name(x)
                }
            )*
        }
    }
}

macro_rules! impl_m {
    ($($name:ident),*) => {
        impl MGHInit {
            $(
                pub fn $name() -> Vec<f64> {
                    problems::$name::init()
                }
            )*
        }
        impl MGH {
            $(
                pub fn $name(&self, x: &[f64]) -> f64 {
                    problems::$name::$name(x, self.m)
                }
            )*
        }
    }
}

macro_rules! impl_nm {
    ($($name:ident),*) => {
        impl MGHInit {
            $(
                pub fn $name(n: usize) -> Vec<f64> {
                    problems::$name::init(n)
                }
            )*
        }
        impl MGH {
            $(
                pub fn $name(&self, x: &[f64]) -> f64 {
                    problems::$name::$name(x, self.m)
                }
            )*
        }
    }
}

macro_rules! impl_min_static {
    ($($name:ident),*) => {
        impl MGHMin {
            $(
                pub fn $name() -> Vec<f64> {
                    problems::$name::min()
                }
            )*
        }
    }
}

macro_rules! impl_min_n {
    ($($name:ident),*) => {
        impl MGHMin {
            $(
                pub fn $name(n: usize) -> Vec<f64> {
                    problems::$name::min(n)
                }
            )*
        }
    }
}
