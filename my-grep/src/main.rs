use std::{fs::File, io::{BufRead, BufReader}};

use clap::Parser;

// #[allow(warnings)]
// mod bindings;

#[derive(Debug, Parser)]
struct Cli {
    pattern: String,
    file_name: String
}

fn start(cli: Cli) -> anyhow::Result<()> {
    let file = File::open(&cli.file_name)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        // lineはResult<String,Error>が束縛されている
        // 下記のように再束縛することで文字として扱う
        let line = line?;
        if line.contains(&cli.pattern) {
            println!("{line}");
        }
    }
    Ok(())
}

fn main() {
    // コマンドライン引数を解釈, Cliオブジェクトを返す
    let cli = Cli::parse();
    if let Err(e) = start(cli) {
        println!("Error(エラー): {e}");
    }
}
