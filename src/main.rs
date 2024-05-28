mod crawler;
mod server;
mod model;
mod tui;

use server::Server;
use model::Model;
use tui::Tui;

use clap::Parser;

use std::fs;


#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Options {
    /// seed at which the web crawler starts at.
    #[arg(long, default_value_t = String::new())]
    seed: String,

    /// serve search engine localy
    #[arg(long, default_value = None)]
    serve: Option<String>,

    /// file to output
    #[arg(short, long, default_value_t = String::from("model.json"))]
    output: String,

    /// when given a seed URL of http://llama.org/hamster/monkey/page.html,
    /// it will attempt to crawl /hamster/monkey/, /hamster/, and /.
    #[arg(short, long, default_value_t = false)]
    ascending: bool,

    /// if limit is set then the web crawler will limit it self
    /// to only crawl subdomains of the starting seed.
    #[arg(short, long, default_value_t = false)]
    limit: bool,

    /// specifies the thread limit.
    #[arg(short, long, default_value_t = 90)]
    threads: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::parse();

    if let Some(path) = options.serve {
        let bytes = fs::read(path)?;
        let model: Model = serde_json::from_slice(&bytes)?;

        let mut server = Server::new(model).map_err(|err| err.to_string())?;

        server.run()?;
    } else {
        let mut tui = Tui::new(options)?;

        tui.run()?;
    }

    Ok(())
}


