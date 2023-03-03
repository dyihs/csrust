fn replace_space(s: String) -> String {
    let mut result = String::new();
    for c in s.chars() {
        if c == ' ' {
            result.push_str("%20");
        } else {
            result.push(c);
        }
    }
    result
}


fn main() {
    let s = "hello world.";
    println!("{}", replace_sapce(s))
}