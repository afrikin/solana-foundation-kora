//! Fee policy.

use crate::config::Config;

/// Returns the fee when it is inside the configured ceiling.
pub fn check_fee(cfg: &Config, lamports: u64) -> Result<u64, String> {
    if lamports > cfg.max_lamports {
        return Err(format!("fee {lamports} exceeds ceiling {}", cfg.max_lamports));
    }
    Ok(lamports)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(max: u64) -> Config { Config { max_lamports: max, ..Default::default() } }

    #[test]
    fn accepts_a_fee_at_the_ceiling() {
        assert_eq!(check_fee(&cfg(10), 10).unwrap(), 10);
    }

    #[test]
    fn refuses_a_fee_above_the_ceiling() {
        assert!(check_fee(&cfg(10), 11).is_err());
    }

    #[test]
    fn refuses_u64_max() {
        assert!(check_fee(&cfg(1_000), u64::MAX).is_err());
    }

    #[test]
    fn accepts_zero() {
        assert_eq!(check_fee(&cfg(10), 0).unwrap(), 0);
    }

    #[test]
    fn refuses_everything_above_a_zero_ceiling() {
        assert!(check_fee(&cfg(0), 1).is_err());
    }
}
