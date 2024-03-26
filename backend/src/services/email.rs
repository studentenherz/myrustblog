use lettre::{
    message::{header::ContentType, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use std::error::Error;

#[derive(Clone)]
pub struct Emailer {
    smtp_client: AsyncSmtpTransport<Tokio1Executor>,
    from_email: String,
}

#[derive(Debug)]
pub struct SmtpConnectionError;

impl Emailer {
    pub fn new(
        smtp_server: &str,
        smtp_username: &str,
        smtp_password: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let from_email = String::from(smtp_username);

        let creds = Credentials::new(smtp_username.to_string(), smtp_password.to_string());

        let smtp_client = AsyncSmtpTransport::<Tokio1Executor>::relay(smtp_server)?
            .credentials(creds)
            .build();

        Ok(Self {
            smtp_client,
            from_email,
        })
    }

    pub async fn test_connection(&self) -> Result<(), SmtpConnectionError> {
        match self.smtp_client.test_connection().await {
            Ok(true) => Ok(()),
            _ => Err(SmtpConnectionError {}),
        }
    }

    async fn send_email(
        &self,
        to: &str,
        subject: &str,
        plain_text: &str,
        html: &str,
    ) -> Result<(), Box<dyn Error>> {
        let email = Message::builder()
            .from(self.from_email.parse()?)
            .to(to.parse()?)
            .subject(subject)
            .multipart(
                MultiPart::alternative() // This is composed of two parts.
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_PLAIN)
                            .body(plain_text.to_string()), // Every message should have a plain text fallback.
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_HTML)
                            .body(String::from(html)),
                    ),
            )?;

        self.smtp_client.send(email).await?;
        Ok(())
    }

    pub async fn send_confirmation_email(
        &self,
        to: &str,
        link: &str,
    ) -> Result<(), Box<dyn Error>> {
        let plain_text = format!(include_str!("templates/confirmation.txt"), link = link);
        let html = format!(include_str!("templates/confirmation.html"), link = link);

        self.send_email(to, "Registration confirmation", &plain_text, &html)
            .await
    }
}
