//! Relayer configuration.

#[derive(Debug, Clone)]
pub struct Config {
    pub rpc_url: String,
    pub fee_payer: String,
}

impl Default for Config {
    fn default() -> Self {
        Self { rpc_url: "http://127.0.0.1:8899".into(), fee_payer: String::new() }
    }
}
