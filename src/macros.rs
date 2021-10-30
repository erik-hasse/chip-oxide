#[macro_export]
macro_rules! instructions {
    ( $( $key:literal $func:item )* ) => {
        $(
            $func
        )*
    };
}

pub(crate) use instructions;
