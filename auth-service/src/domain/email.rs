pub struct Email(String);

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Email {
    pub fn parse(&self) -> Result<&str, String> {
        let email_address = self.as_ref();

        if email_address.contains("@") {
            Ok(email_address)
        } else {
            Err("failed".to_owned())
        }
    }
}
