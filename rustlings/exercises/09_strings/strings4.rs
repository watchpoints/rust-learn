// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    // &str 类型
    string_slice("blue");

    // String 类型
    string("red".to_string());

    // String 类型
    string(String::from("hi"));

    // String 类型
    string("rust is fun!".to_owned());

    // String 类型
    string("nice weather".into());

    // String 类型
    string(format!("Interpolation {}", "Station"));

    // &str 类型
    string_slice(&String::from("abc")[0..1]);

    // &str 类型
    string_slice("  hello there ".trim());

    // String 类型
    string("Happy Monday!".replace("Mon", "Tues"));

    // String 类型
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
