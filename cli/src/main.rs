use structopt::StructOpt;
use anyhow::{Context, Result};

#[derive(StructOpt)]
#[derive(Debug)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}



fn main() -> Result<()> {
  let args = Cli::from_args();
  let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{:?}`", &args.path))?;

  for line in content.lines() {
    if line.contains(&args.pattern) {
        println!("{}", line);
    }
  }
  Ok(())
}

/*
* Tutorial https://rust-cli.github.io/book/tutorial/index.html
* Refactor to use https://doc.rust-lang.org/1.39.0/std/io/struct.BufReader.html to 
* prevent entire file from being loaded into memory
*/