use leptos::*;

use crate::{model::User, repository::LoggedInRepository};

use super::use_identity;

#[tracing::instrument(level = "trace")]
pub async fn use_user() -> Option<User> {
    let Ok(identity) = use_identity() else {
        tracing::debug!("no identity attached to current context");
        return None;
    };

    let Ok(session_id) = identity.id() else {
        tracing::error!("failed to get session id!");
        return None;
    };

    LoggedInRepository::find_user_via_session(&session_id).await
}
