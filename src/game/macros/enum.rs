macro_rules! define_u8_enum {
    (
        $vis:vis enum $name:ident {
            $($variant:ident = $value:expr),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        $vis enum $name {
            $($variant),*,
            Unknown(u8),
        }

        impl From<u8> for $name {
            fn from(value: u8) -> Self {
                match value {
                    $($value => $name::$variant),*,
                    other => {
                        log::warn!("Unknown value for {}: {}", stringify!($name), other);
                        $name::Unknown(other)
                    }
                }
            }
        }

        impl From<$name> for u8 {
            fn from(value: $name) -> u8 {
                match value {
                    $( $name::$variant => $value, )*
                    $name::Unknown(v) => v,
                }
            }
        }
    };
}

pub(crate) use define_u8_enum;
