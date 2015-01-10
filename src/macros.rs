
#[macro_export]
macro_rules! make_uniforms {
    () => {
        $crate::uniforms::EmptyUniforms
    };

    ($field:ident: $value:expr) => {
        $crate::uniforms::UniformsStorage::new(stringify!($field), $value)
    };

    ($field1:ident: $value1:expr, $($field:ident: $value:expr),+) => {
        {
            let uniforms = $crate::uniforms::UniformsStorage::new(stringify!($field1), $value1);
            $(
                uniforms.add(stringify!($field), $value);
            )+
            uniforms
        }
    };
}
/*
#[macro_export]
macro_rules! attributes {
    ($(#[$attr:meta])* struct $struct_name:ident {
        $($(#[$field_attr:meta])* $field:ident: $t:ty),*
    }) => {
        #[derive(Copy)]
        $(#[$attr])*
        pub struct $struct_name {
            $(
                $($field_attr)* pub $field: $t
            ),+
        }

        impl $crate::vertex_buffer::Vertex for $struct_name {
            fn build_bindings(_: Option<Self>) -> $crate::vertex_buffer::VertexFormat {
                vec![
                    $(
                        (
                            stringify!($field),
                            {
                                let dummy: &$struct_name = unsafe { ::std::mem::transmute(0u) };
                                let dummy_field = &dummy.$field;
                                let dummy_field: usize = unsafe { ::std::mem::transmute(dummy_field) };
                                dummy_field
                            },
                            $crate::vertex_buffer::Attribute::get_type(None::<$t>)
                        )
                    )+
                ]
            }
        }
    }
}*/
