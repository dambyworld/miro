use anyhow::{Result, anyhow, bail};

use crate::model::{ProviderKind, SessionRecord};
use crate::provider::{SessionProvider, build_providers};

pub struct SessionManager {
    providers: Vec<Box<dyn SessionProvider>>,
}

impl SessionManager {
    pub fn discover() -> Result<Self> {
        Ok(Self {
            providers: build_providers()?,
        })
    }

    pub fn list_sessions(
        &self,
        provider_filter: Option<ProviderKind>,
    ) -> Result<Vec<SessionRecord>> {
        let mut sessions = Vec::new();
        for provider in &self.providers {
            if provider_filter.is_some_and(|kind| kind != provider.kind()) {
                continue;
            }
            sessions.extend(provider.list_sessions()?);
        }
        sessions.sort_by(|left, right| {
            right
                .updated_at
                .cmp(&left.updated_at)
                .then_with(|| left.title.cmp(&right.title))
        });
        Ok(sessions)
    }

    pub fn find_session(
        &self,
        session_id: &str,
        provider_filter: Option<ProviderKind>,
    ) -> Result<SessionRecord> {
        let matches: Vec<_> = self
            .list_sessions(provider_filter)?
            .into_iter()
            .filter(|session| session.session_id == session_id)
            .collect();
        match matches.as_slice() {
            [session] => Ok(session.clone()),
            [] => bail!("session not found: {session_id}"),
            _ => Err(anyhow!(
                "session id is ambiguous, pass --provider: {session_id}"
            )),
        }
    }

    pub fn delete_session(&self, session: &SessionRecord) -> Result<()> {
        let provider = self
            .providers
            .iter()
            .find(|provider| provider.kind() == session.provider)
            .ok_or_else(|| anyhow!("missing provider {}", session.provider))?;
        provider.delete_session(session)
    }
}
