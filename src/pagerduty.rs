use serde::Serialize;

#[derive(Debug, Serialize)]
struct TriggerEventLink {
    href: String,
    text: String,
}

#[derive(Debug, Serialize)]
struct TriggerEventPayload {
    summary: String,
    source: String,
    severity: &'static str,
}

#[derive(Debug, Serialize)]
pub struct TriggerEvent {
    event_action: String,
    routing_key: String,
    dedup_key: String,
    links: Vec<TriggerEventLink>,
    payload: TriggerEventPayload,
}

fn send_event(event: &TriggerEvent) -> Result<String, u16> {
    let response = ureq::post("https://events.pagerduty.com/v2/enqueue")
        .send_string(serde_json::to_string(event).map_err(|_| 500u16)?.as_str());
    if response.ok() {
        Ok(event.dedup_key.clone())
    } else {
        Err(response.status())
    }
}

impl TriggerEvent {
    pub fn new(trace: String, sender: String, routing_key: String) -> Self {
        Self {
            event_action: "trigger".to_owned(),
            dedup_key: format!(
                "{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos()
            ),
            links: vec![],
            payload: TriggerEventPayload {
                summary: trace,
                source: sender,
                severity: "critical",
            },
            routing_key,
        }
    }
    pub fn send(&self) -> Result<String, u16> {
        send_event(&self)
    }
}
