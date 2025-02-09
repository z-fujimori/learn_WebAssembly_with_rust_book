use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    wasm_file: String
}

fn start(args: Args) -> anyhow::Result<()> {
    Ok(())
}

fn main() {
    let args = Args::parse();

    if let Err(e) = start(args) {
        println!("{:?}", e);
    }
}
