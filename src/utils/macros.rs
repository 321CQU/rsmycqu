macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| regex::Regex::new($re).unwrap());
        &RE
    }};
}

/// 生成支持fallback的serde结构体以便于字段的兼容
macro_rules! serde_fallback {
    ($struct_name:ident, $type:ty, $main_field:ident, fallback = [ $($fields:ident),+ ]$(, apply = [ $(#[$meta:meta]),+ ])?) => {
        // 处理 apply 属性（如果有）
        $(
            $(
            #[$meta]
            )*
        )?
        #[serde_with::serde_as]
        #[serde_with::skip_serializing_none]
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(non_snake_case)]
        struct $struct_name {
            $main_field: Option<$type>,
            $(
                $fields: Option<$type>,
            )+
        }

        impl From<Option<$type>> for $struct_name {
            fn from(value: Option<$type>) -> Self {
                Self {
                    $main_field: value,
                    $(
                        $fields: None,
                    )+
                }
            }
        }

        impl From<$struct_name> for Option<$type> {
            fn from(field: $struct_name) -> Self {
                field.$main_field
                    $(.or(field.$fields))+
            }
        }
    };
}
