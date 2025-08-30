use std::{
    io::{self, BufReader, prelude::*},
    process::{self, Command, Stdio},
};

use clap::Parser;
use tempfile::NamedTempFile;

#[derive(Parser, Debug)]
#[command(name = "rusty-caway", version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = 70)]
    bars: u32,

    #[arg(short, long, default_value_t = 55)]
    framerate: u32,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    // Cava configuration
    let cava_config_content = format!(
        r#"[general]
mode = waves
framerate = {}
bars = {}
lower_cutoff_freq = 50
higher_cutoff_freq = 15000
autosens = 1

[output]
channels = mono
method = raw
data_format = ascii
ascii_max_range = 7

[smoothing]
integral = 77
gravity = 100
ignore = 0
noise_reduction = 0.77
"#,
        cli.framerate, cli.bars
    );

    let mut config_file = NamedTempFile::new()?;
    config_file.write_all(cava_config_content.as_bytes())?;
    let config_path = config_file.path();

    let mut cava_process = match Command::new("cava")
        .arg("-p")
        .arg(config_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(process) => process,
        Err(e) => {
            eprintln!("error executing cava. ¿is cava in your PATH? Error: {}", e);
            process::exit(1);
        }
    };

    let stdout = cava_process
        .stdout
        .take()
        .expect("couldn't capture cava's stdout");
    let reader = BufReader::new(stdout);

    const BAR_CHARS: &[char] = &['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

    for line_result in reader.lines() {
        let line = line_result?;

        let processed_line: String = line
            .split(';')
            .filter_map(|s| s.parse::<usize>().ok())
            .filter_map(|i| BAR_CHARS.get(i))
            .collect();

        println!("{}", processed_line);
    }

    Ok(())
}
