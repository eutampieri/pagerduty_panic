use std::panic;
mod pagerduty;

pub fn register_handler(pagerduty_token: String, pagerduty_source: String) {
    let def_panic_handler = panic::take_hook();

    panic::set_hook(Box::new(move |x| {
        let event = pagerduty::TriggerEvent::new(
            format!("{} {:?}", x, x),
            pagerduty_source.clone(),
            pagerduty_token.clone(),
        );
        if event.send().is_err() {
            eprintln!("Failed to send PagerDuty event");
        } else {
            eprintln!("Created PagerDuty event");
        }
        def_panic_handler(x);
    }));
}
