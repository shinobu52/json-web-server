use clap::{Parser, Subcommand, ArgEnum};
use reqwest::{blocking};
use std::io;

#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,
)]
struct Cli {
    #[clap(subcommand)]
    subcommand: SubCommands,
    /// server url
    #[clap(short = 's', long = "server", value_name = "URL", default_value = "localhost:3000")]
    server: String,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    /// get logs
    #[clap(arg_required_else_help = true)]
    Get {
        /// log format
        #[clap(
            short = 'f',
            long = "format",
            required = true,
            possible_values = ["csv", "json"],
            ignore_case = true,
            arg_enum,
        )]
        format: Format,
    },
    /// post logs, taking input from stdin
    Post,
}

#[derive(Debug, Clone, ArgEnum)]
enum Format {
    Csv,
    Json,
}

fn main() {
    let cli = Cli::parse();

    let client = blocking::Client::new();
    let api_client = ApiClient { server: cli.server, client };

    match cli.subcommand {
        SubCommands::Get { format } => {
            match format {
                Format::Csv => unimplemented!(),
                Format::Json => do_get_json(&api_client)
            }
        },
        SubCommands::Post => do_post_csv(&api_client)
    }
}

struct ApiClient {
    server: String,
    client: blocking::Client,
}

impl ApiClient {
    fn post_logs(&self, req: &api::logs::post::Request) -> reqwest::Result<()> {
        self.client
            .post(&format!("http://{}/logs", &self.server))
            .json(req)
            .send()
            .map(|_| ())
    }

    fn get_logs(&self) -> reqwest::Result<api::logs::get::Response> {
        self.client
            .get(&format!("http://{}/logs", &self.server))
            .send()?
            .json()
    }
}

fn do_post_csv(api_client: &ApiClient) {
    let reader = csv::Reader::from_reader(io::stdin());
    for log in reader.into_deserialize::<api::logs::post::Request>() {
        let log = match log {
            Ok(log) => log,
            Err(e) => {
                eprintln!("[WARNING] failed to parse a line, skipping: {}", e);
                continue
            }
        };
        api_client.post_logs(&log).expect("api request failed");
    }
}

fn do_get_json(api_client: &ApiClient) {
    let res = api_client.get_logs().expect("api request failed");
    let json_str = serde_json::to_string(&res).unwrap();

    println!("{}", json_str)
}
