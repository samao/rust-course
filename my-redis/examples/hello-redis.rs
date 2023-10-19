use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    client.set("hello", "world".into()).await?;

    if let Some(result) = client.get("hello").await? {
        println!("从服务器获取到的结果={:?}", result.slice(..));
    } else {
        println!("不存在KEY");
    }

    Ok(())
}
