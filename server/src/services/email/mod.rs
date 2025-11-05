use lettre::{SmtpTransport, Transport};
use tracing::*;

pub mod error;
// pub mod template;

pub struct EMailService {
    transport: SmtpTransport,
}

impl EMailService {
    pub fn new(config: &crate::config::ServerEMailConfig) -> error::ResultNew<Self> {
        info!("Initializing EMail service...");
        let mut transport_builder = if config.use_starttls.unwrap_or(true) {
            SmtpTransport::starttls_relay(&config.server)
        } else {
            warn!("EMail service is configured to not use STARTTLS.");
            SmtpTransport::relay(&config.server)
        }
        .map_err(|err| error::ErrorNew::LettreError { inner: err })?;

        // Add credentials if both username and password are provided
        if let (Some(username), Some(password)) = (&config.username, &config.password) {
            transport_builder = transport_builder.credentials(
                lettre::transport::smtp::authentication::Credentials::new(
                    username.clone(),
                    password.clone(),
                ),
            );
        } else {
            warn!("EMail service is configured without authentication credentials.");
        }

        Ok(Self {
            transport: transport_builder.build(),
        })
    }

    pub async fn send(&self, email: lettre::Message) -> error::Result<()> {
        self.transport
            .send(&email)
            .map_err(|err| error::Error::LettreError { inner: err })?;
        Ok(())
    }
}
