use clap::Parser;
use ityfuzz::fuzzers::basic_fuzzer;
use std::path::PathBuf;

/// CLI for ItyFuzz
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Glob pattern to find contracts
    #[arg(short, long)]
    contract_glob: String,
}

fn main() {
    let args = Args::parse();
    basic_fuzzer::dummyfuzzer(
        PathBuf::from("./tmp/corpus"),
        PathBuf::from("./tmp/objective"),
        PathBuf::from("./tmp/log"),
        &String::from(args.contract_glob),
    );
}