use serde::Serialize;

#[derive(Clone, clap::ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
}

pub fn print(value: &impl Serialize, format: &OutputFormat) {
    match format {
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(value).unwrap_or_default();
            println!("{json}");
        }
        OutputFormat::Table => {
            let json = serde_json::to_string_pretty(value).unwrap_or_default();
            println!("{json}");
        }
    }
}
