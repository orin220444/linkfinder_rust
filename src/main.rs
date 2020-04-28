use structopt::StructOpt;
extern crate serde;
extern crate serde_json;

/// searches links in telegram message
#[derive(StructOpt)]
struct Cli{
/// The path to the file with telegram entities
#[structopt(parse(from_os_str))]
path: std::path::PathBuf,
}
fn main() {
let args = Cli::from_args();
///get all the data from the file to a variable
let content = std::fs::read_to_string(&args.path)
.expect("could not read file");
println!("{}", content);
/// create a struct to clone data from the object in the array
use serde_json::Number;
struct Entity {
offset: Number,
length: Number,
type_is: String,
url:String,
}
/// create a vec of structs of entities
use std::fmt::Debug;
#[derive(Debug)]
let entities: Vec<Entity> = 
serde_json::from_str(&content)?;//.expect("JSON not well-formatted");
println!("entities: {:#?}",entities);
for i in entities {
println!("{:#?}", [i]);
}

}
