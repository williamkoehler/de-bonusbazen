use std::{collections::HashMap, sync::Arc};

use lettre::{Address, message::Mailbox};
use tracing::*;

use crate::{
    services::{email::EMailService, jinja::JinjaService},
    state::Config,
    users::{UserManager, model::User},
};

#[derive(Clone)]
pub struct UserJobs {
    config: Arc<Config>,
    user_manager: UserManager,
    email_service: EMailService,
    jinja_service: JinjaService,
}

impl UserJobs {
    pub fn new(
        config: Arc<Config>,
        user_manager: UserManager,
        email_service: EMailService,
        jinja_service: JinjaService,
    ) -> Self {
        Self {
            config,
            user_manager,
            email_service,
            jinja_service,
        }
    }

    pub async fn handle_user_registrations_job(&self) -> anyhow::Result<()> {
        info!("handling user registrations...");

        // Get unauthenticated users
        let users = self.user_manager.users(false, false, true).await?;

        for user in users {
            let last_timestamp = self.user_manager.verification_timestamp(user.id).await?;

            let needs_email = match last_timestamp {
                Some(timestamp) => {
                    let duration = chrono::Utc::now() - timestamp;
                    duration.num_hours() >= 2
                }
                None => true,
            };

            if needs_email {
                let user_jobs = self.clone();
                tokio::spawn(async move {
                    if let Err(err) = user_jobs.handle_user_registration_job(user).await {
                        error!("failed to handle user registration: {}", err);
                    }
                });
            }
        }

        info!("successfully handled user registrations");

        Ok(())
    }

    pub async fn handle_user_registration_job(&self, user: User) -> anyhow::Result<()> {
        info!(id = user.id, "handling user registration...");

        let email = match user.email.as_deref() {
            Some(email) => email.to_string(),
            None => {
                warn!(id = user.id, "user has no email, skipping registration");
                return Ok(());
            }
        };

        // Generate unique token
        let token = {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|err| {
                    error!(id = user.id, "time went backwards: {}", err);
                    err
                })?
                .as_secs();

            let claims = crate::users::helper::JwtClaims {
                id: user.id,
                rights: user.rights.into(),
                exp: (now + 3600) as usize,
            };

            jsonwebtoken::encode(
                &jsonwebtoken::Header::default(),
                &claims,
                &jsonwebtoken::EncodingKey::from_secret(
                    self.config.jwt.verification_secret.as_ref(),
                ),
            )
            .map_err(|err| {
                error!(id = user.id, "failed to generate verification jwt: {}", err);
                err
            })?
        };

        // Create verification url
        let verification_url = format!("{}/api/register/{}", self.config.access_host, token);

        // Add verification
        self.user_manager
            .add_verification(user.id)
            .await
            .map_err(|err| {
                warn!(id = user.id, "failed to add verification: {}", err);
                err
            })?;

        info!(
            id = user.id,
            name = user.name,
            email = user.email,
            "register user with {}",
            verification_url
        );

        let id = user.id.to_string();
        let nickname = user.nickname.clone().unwrap_or_else(|| user.name.clone());

        // Render email template
        let body = {
            let mut context: HashMap<&str, &str> = HashMap::new();

            context.insert("id", &id);
            context.insert("name", &user.name);
            context.insert("nickname", &nickname);
            context.insert("email", &email);
            context.insert("token", &token);

            self.jinja_service
                .render_template("verification.email.txt", &context)?
        };

        // Send register email
        let message = lettre::Message::builder()
            .from(Mailbox::new(
                Some("Register".to_string()),
                lettre::Address::new("no-reply", "no-reply").unwrap(),
            ))
            .to(Mailbox::new(
                Some(user.nickname.clone().unwrap_or_else(|| user.name.clone())),
                Address::try_from(email)?,
            ))
            .subject("Verify your account")
            .body(body)?;

        self.email_service.send(message).await.map_err(|err| {
            error!(id = user.id, "failed to send verification email: {}", err);
            err
        })?;

        info!(id = user.id, "successfully handled user registration");

        if let Err(err) = self
            .user_manager
            .set_verification_timestamp(user.id, chrono::Utc::now())
            .await
        {
            error!("failed to set user registration timestamp: {}", err);
        }

        Ok(())
    }
}
