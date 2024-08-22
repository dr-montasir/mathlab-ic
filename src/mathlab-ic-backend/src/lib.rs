use mathlab::math;
use serde_json::json;

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::query]
fn accept_number_as_text_get_sqr_as_text(num_as_str: String) -> String {
    let sqr = math::fix64(math::sqr(num_as_str.parse::<f64>().unwrap()));

    format!(
        "The square of ({}) is ({}). The result comes from the MathLab library: https://crates.io/crates/mathlab",
        num_as_str,
        sqr.to_string()
    )
}

#[ic_cdk::query]
fn accept_float64_get_sqr_as_text(num: f64) -> String {
    let sqr = math::to_fixed(math::sqr(num), 14);

    format!(
        "The square of ({}) is ({}). The result comes from the MathLab library: https://crates.io/crates/mathlab",
        num,
        sqr.to_string()
    )
}

#[ic_cdk::query]
fn accept_float64_get_sqr_as_number(num: f64) -> f64 {
    math::fix(math::sqr(num), 14)
}

#[ic_cdk::query]
fn accept_2floats_get_add_as_text(x: f64, y: f64) -> String {
    let add = math::fix64(math::add(x, y));

    format!(
        "The addition of ({} and {}) is ({}). The result comes from the MathLab library: https://crates.io/crates/mathlab",
        x,
        y,
        add.to_string()
    )
}

#[ic_cdk::query]
fn accept_2floats_get_add_as_number(x: f64, y: f64) -> f64 {
    math::fix64(math::add(x, y))
}

#[ic_cdk::query]
fn accept_args_get_range_as_string_json(x: f64, step: f64) -> String {
    // The maximum size for the range function is 1 million elements.
    // The maximum size for the string field is (325,682 items), (2,934,562 bytes), (2,865.78 kilobytes), or (2.8 megabytes).
    // Set the limit to 100 for a vector length.
    let size = 100;

    // The order can be either 'asc' or 'desc'.
    let order = "asc";

    let vector = math::range(x, step, size, &order);

    json!({
        "result": vector,
        "source": "📏 MathLab 📦 Library",
        "url": "https://crates.io/crates/mathlab"
    })
    .to_string()
}
