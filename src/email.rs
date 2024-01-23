use lettre::message::header::ContentType;
use lettre::message::MessageBuilder;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

#[derive(Clone)]
pub struct Mail(SmtpTransport, MessageBuilder);

impl Mail {
    pub fn new() -> Self {
        let creds = Credentials::new("thankslyh".to_string(), "ahnfqsbvdfgddref".to_string());
        Mail(
            SmtpTransport::relay("smtp.gmail.com")
                .unwrap()
                .credentials(creds)
                .build(),
            Message::builder()
                .from("验证码 <thankslyh@gmail.com>".parse().unwrap())
                .header(ContentType::TEXT_PLAIN),
        )
    }

    pub fn send(&self, email: &Message) -> anyhow::Result<()> {
        let _ = self.0.send(email)?;
        Ok(())
    }

    pub fn send_text(&self, to: String, content: String) -> anyhow::Result<()> {
        let to = format!("<{}>", to);
        let msg = self.1.clone().to(to.parse().unwrap()).body(content)?;
        let _ = self.0.send(&msg)?;
        Ok(())
    }
}

impl AsRef<SmtpTransport> for Mail {
    fn as_ref(&self) -> &SmtpTransport {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Mail;
    use dotenvy::dotenv;
    use lettre::message::header::ContentType;
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{Message, SmtpTransport, Transport};
    use rand::{thread_rng, Rng};
    use std::env;

    #[test]
    fn test_env() {
        dotenv().ok();
        let s = env::var("EMAIL_SMTP").expect("暂无 email 的配置");
        println!("{}", s)
    }

    #[test]
    fn test_email() {
        let email = Message::builder()
            .from("天天开心 <thankslyh@gmail.com>".parse().unwrap())
            .to("新年快乐 <thankslyh@126.com>".parse().unwrap())
            .subject("Happy new year")
            .header(ContentType::TEXT_PLAIN)
            .body(String::from("Be happy!"))
            .unwrap();

        let creds = Credentials::new("thankslyh".to_owned(), "ahnfqsbvdfgddref".to_owned());

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {e:?}"),
        }
    }

    #[test]
    fn test_email_text() {
        let email_server = Mail::new();

        if let Ok(_) = email_server.send_text("thankslyh@126.com".to_string(), "776542".to_string())
        {
            println!("send success");
        }
    }
}
