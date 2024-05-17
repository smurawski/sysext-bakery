use crate::cli::RenderCli;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn render_templates(cli: &RenderCli) {
    let mut render_file = BakeCliRenderFile::new();

    if cli.template_file.is_some() {
        let mut file = BakeCliRenderFileValues::default();
        file.template_path = cli.template_file.as_ref().unwrap().clone();
        file.output_path = cli.output_file.clone();
        let mut values = HashMap::new();
        for value in cli.values.iter() {
            let split: Vec<&str> = value.split('=').collect();
            values
                .insert(split[0].to_string(), split[1].to_string());
        }
        file.values = Some(values);
        render_file.files = vec![file];
    }
    else {
        let config = if let Some(config_file_path) = &cli.config_file {
            std::fs::read_to_string(&config_file_path).unwrap()
        }
        else {
            std::fs::read_to_string("./config.yml").unwrap()
        };
        render_file = BakeCliRenderFile::from_yaml(&config);
    }
    render_file.render_templates();
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BakeCliRenderFile {
    pub files: Vec<BakeCliRenderFileValues>,
    pub values: Option<HashMap<String, String>>,
}

impl BakeCliRenderFile {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_yaml(yaml: &str) -> Self {
        serde_yaml::from_str(yaml).unwrap()
    }

    pub fn render_templates(&self) {
        let handlebars = Handlebars::new();
        for file in &self.files {
            let template = std::fs::read_to_string(&file.template_path).unwrap();
            let mut values = file.values.clone().unwrap_or_default();
            if let Some(shared_values) = &self.values {
                values.extend(shared_values.clone());
            }
            let rendered = handlebars.render_template(&template, &values).unwrap();
            let output_filename = file
                .output_path
                .clone()
                .unwrap_or(file.template_path.clone());
            std::fs::write(output_filename, rendered).unwrap();
        }
    }
}


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BakeCliRenderFileValues {
    pub template_path: String,
    pub output_path: Option<String>,
    pub values: Option<HashMap<String, String>>,
}