pub fn delete() {
    let mut string_pop = String::from("rust pop 中文!");
    dbg!(string_pop.pop());
    dbg!(string_pop.pop());
    println!("pop delete: {}", string_pop);
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string remove 占:{} 个字节",
        std::mem::size_of_val(&string_remove)
    );
    // 按字符删除
    string_remove.remove(0);
    // string_remove.remove(1); panic
    dbg!(string_remove);
    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(6);
    dbg!(string_truncate);
}
