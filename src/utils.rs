// https://danielkeep.github.io/tlborm/book/mbe-macro-rules.html
#[macro_export]
macro_rules! Getter {
    (struct $name:ident { $($fname:ident : $ftype:ty),* }) => {
        struct $name {
            $($fname : $ftype),*
        }

        impl $name {
            fn "get_"$fname() -> $ftype {
                static NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
                NAMES
            }
            fn field_names() -> &'static [&'static str] {
                static NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
                NAMES
            }
        }
    }
}