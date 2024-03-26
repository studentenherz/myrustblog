#[macro_export]
macro_rules! create_env_struct {
    ($struct_name:ident { $($field:ident),+ }) => {
        #[allow(non_snake_case)]
        #[derive(Clone)]
        struct $struct_name {
            $(pub $field: String,)+
        }

        impl $struct_name {
            fn new() -> Self {
                Self {
                    $($field: std::env::var(stringify!($field)).expect(&format!("Environment variable `{}` is required", stringify!($field))),)+
                }
            }
        }
    };
}
