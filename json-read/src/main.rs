use serde::{Deserialize, Serialize};


#[derive(Serialize,Deserialize)]
struct Paragraph{
    name:String
}

#[derive(Serialize,Deserialize)]
struct Article{
    article:String,
    author:String,
    paragraph:Vec<Paragraph>
}

fn main() {
    let json = r#"
    {
    "article":"how to work with json in rust",
    "author":"kunal",
    "paragraph":[
    {
    "name":"first paragraph"
    },
    {
    "name":"second paragraph"
    },
    {
    "name":"third paragraph"
    }
    ]
    }"#;

    let parser :Article = read_json_typed(json);
    println!("\n\n The name of the first paragraph is : {}", parser.paragraph[0].name);
}

fn read_json_typed(raw_json : &str)->Article{
    let parser:Article = serde_json::from_str(raw_json).unwrap();
    parser
}
