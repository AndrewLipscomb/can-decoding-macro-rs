# Testing Proc Macros for a simple declarative CAN decoder

See the src/lib.rs file for what this does

Mainly as an experiment with proc macro work, and how we can use that for declarative struct annotations for encoding

Usual [Rust install is needed](https://www.rust-lang.org/tools/install), then `cargo test -- --nocapture` to run the sole test case

Clone it and have a play with the values - bear in mind I never got the error reporting during proc gen running nicely - so the errors are a bit obtuse there. Check types, and offsets if it complains