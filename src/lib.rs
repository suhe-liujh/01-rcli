mod opts;
mod process;

pub use opts::*;
pub use process::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_format_display() {
        assert_eq!(format!("{}", OutputFormat::Json), "json");
        assert_eq!(format!("{}", OutputFormat::Yaml), "yaml");
        assert_eq!(format!("{}", OutputFormat::Toml), "toml");
    }

    #[test]
    fn test_output_format_from_str() {
        use std::str::FromStr;

        assert!(matches!(
            OutputFormat::from_str("json"),
            Ok(OutputFormat::Json)
        ));
        assert!(matches!(
            OutputFormat::from_str("yaml"),
            Ok(OutputFormat::Yaml)
        ));
        assert!(matches!(
            OutputFormat::from_str("toml"),
            Ok(OutputFormat::Toml)
        ));
        assert!(OutputFormat::from_str("invalid").is_err());
    }
}
