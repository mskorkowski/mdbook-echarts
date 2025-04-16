mod chart;

use std::io;
use std::path::PathBuf;
use std::process;

use clap::ArgMatches;
use clap::arg;
use clap::value_parser;
use clap::Command;

use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};

use chart::Chart;


pub fn make_app() -> Command {
    Command::new("chart-preprocessor")
        .about("A mdbook preprocessor to auto generate book summary")
        .subcommand(
            Command::new("supports")
                .arg(
                    arg!(<renderer>).required(true).value_parser(value_parser!(PathBuf))
                )
                .about("Check whether a renderer is supported by this preprocessor"),
        )
        .subcommand(
            Command::new("version")
                .about("Print the version of this preprocessor"),
        )
        // .subcommand(
        //     Command::new("help")
        //         .about("help doc for use mdbook-chart preprocessor"),
        // )
}

fn main() {
    let matches = make_app().get_matches();

    let preprocessor = Chart::new();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&preprocessor, sub_args);
    } else if let Some(_sub_args) = matches.subcommand_matches("help") {
        println!("please use mdbook-echarts as a preprocessor in your book.toml");
    } else if let Some(..) = matches.subcommand_matches("version") {
        println!("0.0.7-mskorkowski")
    } else if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer: &PathBuf = sub_args.get_one("renderer").expect("Required argument");
    let renderer = renderer.to_str().unwrap_or("not-supported");
    let supported = pre.supports_renderer(renderer);

    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}
