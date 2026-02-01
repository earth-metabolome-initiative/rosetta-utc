//! WASM tests for rosetta-utc crate.
#![cfg(target_arch = "wasm32")]

use rosetta_utc::TimestampUTC;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_now() {
    let now = TimestampUTC::now();
    assert!(now.timestamp() > 0);
}

#[wasm_bindgen_test]
fn test_conversions() {
    let now = TimestampUTC::now();
    let s = now.to_string();
    assert!(s.len() > 0);

    // Test conversion from string expects RFC3339, which Display does not produce by default.
    // Display produces "YYYY-MM-DD HH:MM:SS UTC"
    // FromStr expects "YYYY-MM-DDTHH:MM:SS+00:00"

    let rfc = now.to_rfc3339();
    let parsed_rfc: TimestampUTC = rfc.parse().unwrap();
    assert_eq!(now, parsed_rfc);
}
