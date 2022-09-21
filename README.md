# PagerDuty panic reporter

This crate registers a panic handler that sends panic messages to PagerDuty.

```rust
    pagerduty_panic::register_handler(
        std::env::var("PAGERDUTY_TOKEN").unwrap(),
        "Application name".to_owned(),
    );
```