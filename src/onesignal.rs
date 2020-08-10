use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NotificationPayload {
    pub app_id: &'static str,
    pub include_player_ids: Vec<String>,
    pub headings: Headings,
    pub contents: Contents,
    pub chrome_web_image: String,
    pub web_url: String,
}

#[derive(Debug, Serialize)]
pub struct Headings {
    pub en: String,
}

#[derive(Debug, Serialize)]
pub struct Contents {
    pub en: String,
}

pub fn send_notification(payload: NotificationPayload) -> Result<()> {
    let res = ureq::post(crate::constants::ONE_SIGNAL_API_ENDPOINT)
        .send_json(serde_json::to_value(payload)?);
    if !res.ok() {
        eprintln!("Failed to send Push Notification!!");
        eprintln!("Status Code: {} {}", res.status(), res.status_text());
        eprintln!("{}", res.into_json()?);
    }
    Ok(())
}
