use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "actixweb4-starter", about = "actixweb4-starter - http rest/ws/ react static server")]

pub enum Cli {
  #[structopt(name = "start-server")]
  StartHttpServer {
    // Use `env` to enable specifying the option with an environment
    // variable. Command line arguments take precedence over env.
    // URL for the API server
    // #[structopt(long, env = "RUST_LOG")]
    // log_level: String,

    // mode cli config file -c

    // BOF : UNCOMMENT to use config
    // // config file: don't use default_value = "./config.json" this will required implicit pass a config file
    // #[structopt(short = "c", long = "config", min_values = 1, max_values = 1)]
    // #[structopt(short, long)]
    // #[structopt(parse(from_os_str))]
    // config_file: Option<std::path::PathBuf>,
    // EOF : UNCOMMENT to use config
  
    // mode cli args

    // BOF : UNCOMMENT to use config
    // // -i | --files
    // #[structopt(short, long)]
    // #[structopt(short = "i", long = "files", min_values = 1)]
    // #[structopt(parse(from_os_str))]
    // input_files: Vec<std::path::PathBuf>,
    // EOF : UNCOMMENT to use config

    // BOF : UNCOMMENT to use config
    // // -f | --filter-file
    // #[structopt(short = "f", long = "filter-file", default_value = r"^.*c3-.*\.log$")]
    // #[structopt(short, long)]
    // filter_file: String,
    // EOF : UNCOMMENT to use config

    // BOF : UNCOMMENT to use config
    // // -l | --filter-line
    // #[structopt(short, long)]
    // #[structopt(short = "l", long = "filter-line", default_value = r"(?i)(.*)")]
    // filter_line: String,
    // EOF : UNCOMMENT to use config
  },
}
