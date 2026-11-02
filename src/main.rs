use kora_relayer::config::Config;
use kora_relayer::signer::{MemorySigner, Signer};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cfg = Config::from_str("");
    match args.first().map(String::as_str) {
        Some("config") => {
            println!("rpc_url = {}", cfg.rpc_url);
            println!("max_lamports = {}", cfg.max_lamports);
        }
        Some("signer") => {
            let s = MemorySigner { key: "payer-1".into() };
            println!("public_key = {}", s.public_key());
        }
        _ => println!("usage: kora <config|signer>"),
    }
}
