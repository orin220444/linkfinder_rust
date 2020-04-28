use structopt::StructOpt;
/// searches links in telegram message
#[derive(StructOpt)]
struct Cli{
/// The path to the file with telegram entities
#[structopt(parse(from_os_str))]
path: std::path::PathBuf,
}
fn main() {
let args = Cli::from_args();
let content = std::fs::read_to_string(&args.path)
.expect("could not read file");
println!("{}", content)
}
