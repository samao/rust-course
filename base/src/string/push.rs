pub fn push() {
    let mut s = String::from("hello ");
    s.push('r');
    println!("追加字符串push() => {}", s);
    s.push_str("ust!");
    println!("追加字符串push_str() => {}", s);
}
