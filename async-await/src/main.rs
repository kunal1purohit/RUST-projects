use error_chain::error_chain;
use anyhow::Result;

// error_chain!{
//     foreign_links{
//         Io(std::io::Error);
//         HttpRequest(reqwest::Error);
//     }
// }

#[tokio::main]
async fn main()->Result<()>{
    let res = reqwest::get("http://httpbin.org/get").await?;
    println!("Status :{}",res.status());
    println!("headers:{:#?}",res.headers());
    let body = res.text().await?;
    println!("body:\n{}",body);

    Ok(())

}
