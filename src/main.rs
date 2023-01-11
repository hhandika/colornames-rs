use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    request_color_names().await?;

    Ok(())
}

async fn request_color_names() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://unpkg.com/color-name-list/dist/colornames.json";
    let resp = reqwest::get(url).await?;

    match resp.status() {
        reqwest::StatusCode::OK => {
            let color_names: Vec<Color> = resp.json().await?;
            write_json_to_file(&color_names)?;
            generate_rust_code(&color_names)?;
        }
        _ => println!("Error"),
    }

    Ok(())
}

fn write_json_to_file(json: &[Color]) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("json")?;
    let file = File::create("json/colors.json")?;
    let mut buff = BufWriter::new(file);

    let json = serde_json::to_string_pretty(&json)?;
    buff.write_all(json.as_bytes())?;

    Ok(())
}

fn generate_rust_code(values: &[Color]) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("src/colors.rs").unwrap();
    let mut buff = BufWriter::new(file);

    writeln!(buff, "use std::collections::HashMap;")?;
    writeln!(buff)?;
    writeln!(buff, "lazy_static! {{")?;
    writeln!(
        buff,
        "    pub static ref COLORS: HashMap<&'static str, &'static str> = {{"
    )?;
    writeln!(buff, "        let mut map = HashMap::new();")?;
    values.iter().for_each(|color| {
        writeln!(
            buff,
            "        map.insert(\"{}\", \"{}\");",
            color.name, color.hex
        )
        .unwrap();
    });
    writeln!(buff, "}}")?;
    writeln!(buff, "}}")?;

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Color {
    name: String,
    hex: String,
}
