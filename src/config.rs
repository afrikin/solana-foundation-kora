//! Relayer configuration.

#[derive(Debug, Clone)]
pub struct Config {
    pub rpc_url: String,
    pub fee_payer: String,
    pub max_lamports: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self { rpc_url: "http://127.0.0.1:8899".into(), fee_payer: String::new(),
               max_lamports: 5_000_000 }
    }
}

impl Config {
    /// Parse a minimal key = "value" file. Unknown keys are ignored on purpose:
    /// older configs keep working when a field is removed.
    pub fn from_str(raw: &str) -> Self {
        let mut cfg = Self::default();
        for line in raw.lines() {
            let line = line.split('#').next().unwrap_or("").trim();
            if line.is_empty() || !line.contains('=') { continue; }
            let (k, v) = line.split_once('=').unwrap();
            let v = v.trim().trim_matches('"');
            match k.trim() {
                "rpc_url" => cfg.rpc_url = v.to_string(),
                "fee_payer" => cfg.fee_payer = v.to_string(),
                "max_lamports" => {
                    if let Ok(n) = v.parse::<u64>() { cfg.max_lamports = n; }
                }
                _ => {}
            }
        }
        cfg
    }
}
