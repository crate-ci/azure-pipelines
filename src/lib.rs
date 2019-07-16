#[cfg(all(test, feature = "opt_in"))]
#[test]
fn opt_in_test() {}

#[cfg(all(test, feature = "opt_out"))]
#[test]
fn opt_out_test() {}

#[cfg(test)]
#[test]
fn always_test() {}

#[cfg(test)]
#[test]
fn require_env() {
    std::env::var("ENV_IS_SET").unwrap();
}

// check that minrust gets set correctly
#[allow(unused_imports)]
use std::sync::atomic::AtomicU64;
