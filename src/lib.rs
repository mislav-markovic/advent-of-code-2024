use std::path::PathBuf;

use eyre::Context;
use tracing_subscriber::EnvFilter;

pub fn init() {
    let format = tracing_subscriber::fmt::format()
        .with_thread_ids(false)
        .with_target(false)
        .with_level(true)
        .with_file(false)
        .with_source_location(false)
        .with_line_number(false)
        .with_ansi(true)
        .compact();

    tracing_subscriber::fmt()
        .event_format(format)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    color_eyre::install().expect("color eyre failed to install");
}

pub fn load_day_input(day_name: impl AsRef<str>) -> eyre::Result<String> {
    const INPUTS_DIR_NAME: &'static str = "inputs";
    let day_name = day_name.as_ref();

    let input_path: PathBuf = [INPUTS_DIR_NAME, day_name].iter().collect();
    let input_path = input_path
        .canonicalize()
        .wrap_err_with(|| format!("failed to expand input file path: {:?}", input_path))?;

    std::fs::read_to_string(input_path.as_path()).wrap_err_with(|| {
        format!("failed to load input data for {day_name} from path {input_path:?}")
    })
}
