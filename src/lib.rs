use villagesql::{InValue, VdfReturn};

fn {{sql_func_name}}_impl(args: &[InValue]) -> VdfReturn {
    match args.first() {
        Some(InValue::String(s)) => VdfReturn::string(s.to_string()),
        Some(InValue::Null) | None => VdfReturn::null(),
        _ => VdfReturn::error("{{sql_func_name}}: expected a STRING argument"),
    }
}

villagesql::extension! {
    funcs: [
        villagesql::func!({{sql_func_name}}_impl, "{{sql_func_name}}", [villagesql::Type::String] -> villagesql::Type::String),
    ]
}
