macro_rules! define_options {
    (
        $struct:ident . $options_field:ident ;
        $(
            $field_name:ident : $field_type:ty = $field_default:expr ,
        )*
        $(
            @with_setters
            $(
                $auto_field_name:ident : $auto_field_type:ty = $auto_field_default:expr
            ),*
        )?
    ) => {
        #[derive(reactive_stores_macro::Store)]
        struct Options {
            $(
                $field_name: $field_type,
            )*
            $(
                $(
                    $auto_field_name: $auto_field_type,
                )*
            )?
        }

        impl Default for Options {
            fn default() -> Self {
                Self {
                    $(
                        $field_name: $field_default,
                    )*
                    $(
                        $(
                            $auto_field_name: $auto_field_default,
                        )*
                    )?
                }
            }
        }

        impl $struct {
            paste::paste! {
                $($(
                    pub fn $auto_field_name(self, value: impl Into< $auto_field_type >) -> Self {
                        use leptos::prelude::Set;
                        self.$options_field.clone().$auto_field_name().set(value.into());
                        self
                    }

                    pub fn [< $auto_field_name _signal >](
                        self,
                        value: impl leptos::prelude::Get<Value = $auto_field_type> + Send + Sync + 'static
                    ) -> Self {
                        use leptos::prelude::{ImmediateEffect, Set, StoredValue};

                        let field = self.$options_field.clone().$auto_field_name();
                        StoredValue::new(ImmediateEffect::new(move || {
                            field.set(value.get());
                        }));

                        self
                    }
                )*)?
            }
        }
    };
}

pub(crate) use define_options;
