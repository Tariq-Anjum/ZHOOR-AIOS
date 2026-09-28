# NON-PRODUCTION ARC-02 comparison fixtures

`valid.jsonl` contains 14 valid requests (one intentional empty-candidate abstention), including the signed 64-bit score endpoints. `invalid.jsonl` contains 15 structured invalid requests plus one malformed JSON line, including values just outside that score domain. The same files feed Rust-first, Python-first, and mixed tests.
