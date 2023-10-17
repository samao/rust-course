pub fn insert() {
    let mut s = String::from("hello rust!");
    s.insert(5, ',');
    println!("插入字符 insert() -> {}, 长度: {}", s, s.len());
    s.insert_str(6, " I like");
    println!("插入字符串 insert_str() -> {}, 长度: {}", s, s.len());
}
