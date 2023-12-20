use error_chain::error_chain;
use std::io::Read;

error_chain!{
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main () -> Result<()>{
let mut res = reqwest::blocking::get("https://httpbin.org/get")?;
let mut req = String::new();
res.read_to_string(&mut req)?;

println!("Status: {}", res.status());
println!("Json: \n{:#?}", res.headers());
println!("get request: \n{}", req);
Ok(())
}