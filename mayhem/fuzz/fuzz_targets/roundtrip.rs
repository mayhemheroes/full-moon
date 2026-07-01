// Additive in-process libFuzzer harness for full-moon (a lossless Lua parser).
//
// This mirrors the crate's OWN upstream fuzz target (full-moon/fuzz/fuzz_targets/
// roundtrip.rs): feed the fuzzer bytes as UTF-8 Lua source into full_moon::parse,
// and on a successful parse assert the crate's lossless round-trip invariant
// (print(parse(code)) == code). Any input that parses but does not print back
// identically is a real full-moon defect (the product), so we do NOT guard it.
// No disk I/O; upstream source is untouched — this crate only CALLS full_moon.
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|code: &str| {
    if let Ok(ast) = full_moon::parse(code) {
        let printed = full_moon::print(&ast);
        assert_eq!(code, printed);
    }
});
