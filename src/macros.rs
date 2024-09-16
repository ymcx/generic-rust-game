#[macro_export]
/// If we didn't have the derive macro for position,
/// this would be a good option to reduce copy pasted code.
/// See usage example in hud.rs
macro_rules! impl_position {
    (<$inner_type:ident> with $position_type:ident for $type:ty) => {
        impl Position<$inner_type> for $type {
            fn position(&self) -> $position_type<$inner_type> {
                self.position
            }
            fn set_position(&mut self, position: $position_type<$inner_type>) {
                self.position.x = position.x;
                self.position.y = position.y;
            }
        }
    };
}

#[macro_export]
macro_rules! impl_display {
    (for $type:ty: $to_print:expr) => {
        impl ::std::fmt::Display for $type {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}", $to_print)
            }
        }
    };
    (for $type:ty: $name:ident) => {
        impl ::std::fmt::Display for $type {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}", self.$name)
            }
        }
    };
    (for $type:ty: method $field:tt) => {
        impl ::std::fmt::Display for $type {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}", self.$field())
            }
        }
    };
}
