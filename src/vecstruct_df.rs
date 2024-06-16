#[macro_export]
macro_rules! struct_to_dataframe {
        ($input:expr, [$($field:ident),+]) => {
            {
                let len = $input.len().to_owned();

                // Extract the field values into separate vectors
                $(let mut $field = Vec::with_capacity(len);)*

                for e in $input.into_iter() {
                    $($field.push(e.$field);)*
                }
                df! {
                    $(stringify!($field) => $field,)*
                }
            }
        };
    }
// pub use struct_to_dataframe;

#[derive(Debug, Clone, PartialEq)]
pub struct TestStruct {
    pub height: u32,
    pub width:  u64,
}
