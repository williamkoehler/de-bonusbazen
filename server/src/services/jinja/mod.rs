use std::collections::HashMap;

use minijinja::*;

pub mod error;

#[derive(Clone)]
pub struct JinjaService {
    env: Environment<'static>,
}

impl JinjaService {
    pub fn new(config: &crate::config::ServerJinjaConfig) -> Self {
        let mut env = Environment::new();
        env.set_loader(path_loader(&config.path));

        Self { env }
    }

    pub fn render_template(
        &self,
        name: &str,
        context: &HashMap<&str, &str>,
    ) -> error::Result<String> {
        let template = self
            .env
            .get_template(name)
            .map_err(|_| error::Error::TemplateNotFound {
                name: name.to_string(),
            })?;

        let rendered = template
            .render(context)
            .map_err(|err| error::Error::JinjaError { inner: err })?;

        Ok(rendered)
    }
}
