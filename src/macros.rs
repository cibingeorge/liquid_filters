#[allow(unused_macros)]
#[macro_export]
macro_rules! call_filter_with_runtime {
    ($runtime: expr, $filter:expr, $input:expr) => {{
        $crate::call_filter_with_runtime!($runtime, $filter, $input, )
    }};
    ($runtime: expr, $filter:expr, $input:expr, $($args:expr),*) => {{
        let positional = Box::new(vec![$(liquid_core::Expression::Literal(liquid_core::value!($args))),*].into_iter());
        let keyword = Box::new(Vec::new().into_iter());
        let args = liquid_core::parser::FilterArguments { positional, keyword };

        let input = liquid_core::value!($input);
        liquid_core::ParseFilter::parse(&$filter, args)
            .and_then(|filter| liquid_core::Filter::evaluate(&*filter, &input, &$runtime))
    }};
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! render_template {
    ($template:expr, $json_value:expr) => {{
        let json_str = serde_json::to_string(&$json_value).unwrap();
        let render_context: liquid::model::Object = serde_json::from_str(&json_str).unwrap();

        let value = $crate::template::Template::parse($template).unwrap();
        let rc = $crate::RenderContext::new();
        let rendered = value.render_with_context(rc, &render_context).unwrap();
        rendered
    }};
}
