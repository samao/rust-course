pub fn concat() {
    let string_append = String::from("Hello ");
    let string_rust = String::from("rust");
    // string_append 移动了
    let result = string_append + &string_rust;

    let mut result = result + "!";
    result += "!!!";

    println!("连接字符串 +->[{}] = [{}]", string_rust, result);
}
