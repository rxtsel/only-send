use crate::store;
use resend_rs::{list_opts::ListOptions, Resend};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Wry};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentEmail {
    pub id: String,
    pub from: String,
    pub to: Vec<String>,
    pub subject: String,
    pub html: Option<String>,
    pub created_at: String,
    #[serde(default)]
    pub cc: Vec<String>,
    #[serde(default)]
    pub bcc: Vec<String>,
    pub reply_to: Option<String>,
}

#[tauri::command]
pub async fn list_sent_emails(
    app: AppHandle<Wry>,
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<Vec<SentEmail>, String> {
    let api_key = store::get_api_key(app.clone())?
        .ok_or_else(|| "[ERROR] API key not configured".to_string())?;

    let resend = Resend::new(&api_key);

    let lim = limit.unwrap_or(12);
    let off = offset.unwrap_or(0);

    // Pedimos hasta offset + limit, para poder "paginar" a mano
    let effective_limit = (lim + off).min(255);
    let list_opts = ListOptions::default().with_limit(effective_limit as u8);

    let response = resend
        .emails
        .list(list_opts)
        .await
        .map_err(|e| format!("[ERROR] Failed to fetch sent emails: {}", e))?;

    let sent_emails: Vec<SentEmail> = response
        .data
        .into_iter()
        .skip(off) // Skip previeous pages
        .take(lim) // Only take 'limit' items
        .map(|email| SentEmail {
            id: email.id.to_string(),
            from: email.from,
            to: email.to,
            subject: email.subject,
            html: email.html,
            created_at: email.created_at,
            cc: email.cc,
            bcc: email.bcc,
            reply_to: email.reply_to.as_ref().and_then(|rt| {
                if rt.is_empty() {
                    None
                } else {
                    Some(rt.join(", "))
                }
            }),
        })
        .collect();

    Ok(sent_emails)
}

#[tauri::command]
pub async fn get_sent_email(app: AppHandle<Wry>, email_id: String) -> Result<SentEmail, String> {
    // Get API key from storage
    let api_key = store::get_api_key(app.clone())?
        .ok_or_else(|| "[ERROR] API key not configured".to_string())?;

    let resend = Resend::new(&api_key);

    // Get specific email by ID
    let email = resend
        .emails
        .get(&email_id)
        .await
        .map_err(|e| format!("[ERROR] Failed to fetch email: {}", e))?;

    Ok(SentEmail {
        id: email.id.to_string(),
        from: email.from,
        to: email.to,
        subject: email.subject,
        html: email.html.or(email.text),
        created_at: email.created_at,
        cc: email.cc,
        bcc: email.bcc,
        reply_to: email.reply_to.as_ref().and_then(|rt| {
            if rt.is_empty() {
                None
            } else {
                Some(rt.join(", "))
            }
        }),
    })
}
