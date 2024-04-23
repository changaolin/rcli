use anyhow::Result;
use csv::Reader;

use super::OutputFormat;
pub fn process_csv(input: &str, output: &str, format: OutputFormat, no_header: bool) -> Result<()> {
    let mut buf = String::new();
    if input == "-" {
        std::io::stdin().read_line(&mut buf)?;
    } else {
        buf = std::fs::read_to_string(input)?;
    }
    let mut ret = vec![];
    let mut rdr = Reader::from_reader(buf.as_bytes());
    if no_header {
        rdr.records().enumerate().for_each(|(_, record)| {
            let json_value = record.unwrap().iter().collect::<serde_json::Value>();
            ret.push(json_value);
        });
    } else {
        let header = rdr.headers()?.clone();
        rdr.records().enumerate().for_each(|(_, record)| {
            let json_value = header
                .iter()
                .zip(record.unwrap().iter())
                .collect::<serde_json::Value>();
            ret.push(json_value);
        });
    }
    let write_data = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };
    if output == "-" {
        print!("{}", write_data);
    } else {
        std::fs::write(output, write_data)?;
    }
    Ok(())
}
