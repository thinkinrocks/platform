use std::sync::Arc;

use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use chrono::Duration;
use expiringmap::ExpiringMap;
use rand::{TryRngCore, rngs::OsRng};
use tokio::sync::RwLock;

#[derive(Default, Clone, PartialEq, Eq)]
pub struct SessionData {
    username: String,
    logout_key: String,
}

#[derive(Default)]
struct SessionsState {
    sessions: ExpiringMap<String, SessionData>,
}

pub struct Sessions {
    sessions: RwLock<SessionsState>,
}

impl Sessions {
    pub fn new() -> Arc<Self> {
        Arc::new(Sessions {
            sessions: RwLock::default(),
        })
    }

    pub async fn create_session(&self, username: String, duration: Duration) -> (String, String) {
        let sessions = &mut self.sessions.write().await.sessions;

        let mut session_key_bytes = [0u8; 32];
        OsRng.try_fill_bytes(&mut session_key_bytes).unwrap();
        let session_key = BASE64_URL_SAFE_NO_PAD.encode(&session_key_bytes);

        let mut logout_key_bytes = [0u8; 32];
        OsRng.try_fill_bytes(&mut logout_key_bytes).unwrap();
        let logout_key = BASE64_URL_SAFE_NO_PAD.encode(&logout_key_bytes);

        sessions.insert(
            session_key.to_string(),
            SessionData {
                username,
                logout_key: logout_key.to_string(),
            },
            duration.to_std().unwrap(),
        );

        (session_key, logout_key)
    }

    pub async fn check_session(&self, session: impl AsRef<str>) -> Option<SessionData> {
        let session_key = session.as_ref();
        let sessions = &self.sessions.read().await.sessions;

        sessions.get(session_key).cloned()
    }

    pub async fn delete_session(&self, session: impl AsRef<str>) -> bool {
        self.sessions
            .write()
            .await
            .sessions
            .remove(session.as_ref())
    }
}
