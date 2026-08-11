#[macro_export]
macro_rules! define_entity {
    (
        $create_name:ident, $name:ident {
            $id_field:ident : $id_type:ty,
            $($field:ident : $ftype:ty),* $(,)?
        }
    ) => {
        #[allow(unused)]
        #[derive(Debug, Clone, PartialEq, sqlx::prelude::FromRow)]
        pub struct $name {
            pub $id_field: $id_type,
            $(pub $field: $ftype,)*
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct $create_name {
            $(pub $field: $ftype,)*
        }
    };
}
