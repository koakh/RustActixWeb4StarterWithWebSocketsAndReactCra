use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "actixweb4-starter", about = "critical-links cryptlex command line interface application")]

pub enum Cli {
  #[structopt(name = "start-server")]
  StartHttpServer {
    // Use `env` to enable specifying the option with an environment
    // variable. Command line arguments take precedence over env.
    /// URL for the API server
    // #[structopt(long, env = "RUST_LOG")]
    // log_level: String,

    /// config file: don't use default_value = "./config.json" this will required implicit pass a config file
    #[structopt(short = "c", long = "config", min_values = 1, max_values = 1)]
    #[structopt(short, long)]
    #[structopt(parse(from_os_str))]
    config_file: Option<std::path::PathBuf>,

    #[structopt(short, long)]
    #[structopt(short = "i", long = "files", min_values = 1)]
    #[structopt(parse(from_os_str))]
    input_files: Vec<std::path::PathBuf>,

    #[structopt(short = "f", long = "filter-file", default_value = r"^.*c3-.*\.log$")]
    #[structopt(short, long)]
    filter_file: String,

    #[structopt(short, long)]
    #[structopt(short = "l", long = "filter-line", default_value = r"(?i)(.*)")]
    filter_line: String, 
  }
}