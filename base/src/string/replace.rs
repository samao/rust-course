pub fn replace() {
    let mut s = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replace = s.replace("rust", "javascript");
    println!("old: {} = new: {}", s, new_string_replace);

    let new_string_replacen = new_string_replace.replacen("javascript", "Go", 1);
    println!("old: {} = new: {}", new_string_replace, new_string_replacen);

    s.replace_range(1..=12, "'m ");
    println!("origin s change to: {}", s);
}
