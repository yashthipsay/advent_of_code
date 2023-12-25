use error_chain::error_chain;

error_chain!{
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()>{
    let res = reqwest::get("http://httpbin.org/get");
    println!("Status: {}", res.status());
    let arguments = res.text().await?;
    println!("Arguments:\n{}", arguments);
    Ok(())
}