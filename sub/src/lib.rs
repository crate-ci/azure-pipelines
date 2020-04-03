#[cfg(test)]
#[test]
fn always_test() {}

#[cfg(test)]
#[test]
fn require_env() {
    std::env::var("SUBENV_IS_SET").unwrap();
}

#[cfg(test)]
#[test]
fn require_setup_file() {
    include_str!("setup.rs");
}

// check that minrust gets set correctly
#[allow(unused_imports)]
use std::sync::atomic::AtomicU64;
