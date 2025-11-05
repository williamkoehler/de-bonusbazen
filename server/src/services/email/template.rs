pub struct EMailTemplate {
    context: Box<minijinja::Template>,
}

impl EMailTemplate {
    pub fn new() -> Self {
        let environment = minijinja::Environment::new();
    }
}
