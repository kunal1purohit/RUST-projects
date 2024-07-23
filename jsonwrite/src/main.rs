use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
struct Paragraph{
    name:String
}

#[derive(Serialize,Deserialize)]
struct Article{
    article:String,
    author:String,
    paragraphs:Vec<Paragraph>
}

fn main(){
    let article:Article=Article{
        article:String::from("how to json in rust"),
        author:String::from("kunal"),
        paragraphs:vec![
            Paragraph{name:String::from("paragraph 1")},
            Paragraph{name:String::from("paragraph 2")},
            Paragraph{name:String::from("paragraph 3")}
        ]
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("the json is {}",json)
}


