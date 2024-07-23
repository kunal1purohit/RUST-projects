use error_chain::error_chain;
use std::io::Read;

error_chain!{
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main()->Result<()>{
    let mut info = reqwest::blocking::get("http://httpbin.org/get")?;
    let mut body = String::new();
    info.read_to_string(&mut body)?;

    println!("Status : {}",info.status());
    println!("Headers: \n {:?}", info.headers());
    println!("Body: \n{}",body);

    Ok(())
}
