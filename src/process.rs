use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

use crate::OutputFormat;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    pub name: String,
    pub position: String,
    #[serde(rename = "DOB")]
    pub dob: String,
    pub nationality: String,
    #[serde(rename = "Kit Number")]
    pub kit_number: u8,
}

#[derive(Serialize)]
struct TomlWrapper {
    data: Vec<HashMap<String, String>>,
}

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let header = reader.headers()?.clone();
    println!("{:?}", header);

    let content = match format {
        OutputFormat::Json => {
            let mut json_res = Vec::new();
            for result in reader.records() {
                let record = result?;
                let json_val: serde_json::Value = header
                    .iter()
                    .zip(record.iter())
                    .map(|(k, v)| (k.to_string(), serde_json::Value::String(v.to_string())))
                    .collect::<serde_json::Map<String, serde_json::Value>>()
                    .into();
                json_res.push(json_val);
            }
            serde_json::to_string_pretty(&json_res)?
        }
        OutputFormat::Yaml => {
            let mut yaml_res = Vec::new();
            for result in reader.records() {
                let record = result?;
                let yaml_val: serde_yaml::Value = header
                    .iter()
                    .zip(record.iter())
                    .map(|(k, v)| {
                        (
                            serde_yaml::Value::String(k.to_string()),
                            serde_yaml::Value::String(v.to_string()),
                        )
                    })
                    .collect::<serde_yaml::Mapping>()
                    .into();
                yaml_res.push(yaml_val);
            }
            serde_yaml::to_string(&yaml_res)?
        }
        OutputFormat::Toml => {
            let mut toml_res = Vec::new();
            for result in reader.records() {
                let record = result?;
                let toml_val: HashMap<String, String> = header
                    .iter()
                    .zip(record.iter())
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect();
                toml_res.push(toml_val);
            }
            let wrapper = TomlWrapper { data: toml_res };
            toml::to_string_pretty(&wrapper)?
        }
    };

    fs::write(output, content)?;
    Ok(())
}
