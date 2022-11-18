#![allow(non_snake_case)]
use clap::{load_yaml, App};
use reveaal::logging::setup_logger;

use reveaal::ProtobufServer::services::query_request::settings::ReduceClocksLevel;
use reveaal::ProtobufServer::services::query_request::Settings;
use reveaal::{
    extract_system_rep, parse_queries, start_grpc_server_with_tokio, xml_parser, ComponentLoader,
    JsonProjectLoader, ProjectLoader, Query, QueryResult, XmlProjectLoader, DEFAULT_SETTINGS,
};
use std::env;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "logging")]
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();
    setup_logger().unwrap();

    if let Some(ip_endpoint) = matches.value_of("endpoint") {
        start_grpc_server_with_tokio(ip_endpoint)?;
    } else {
        start_using_cli(&matches);
    }

    Ok(())
}

fn start_using_cli(matches: &clap::ArgMatches) {
    let (mut comp_loader, queries) = parse_args(matches);

    let mut results = vec![];
    for query in &queries {
        let executable_query = Box::new(
            extract_system_rep::create_executable_query(query, &mut *comp_loader).unwrap(),
        );

        let result = executable_query.execute();

        if let QueryResult::Error(err) = result {
            panic!("{}", err);
        }

        results.push(result);
    }

    println!("\nQuery results:");
    for index in 0..queries.len() {
        results[index].print_result(&queries[index].query.as_ref().unwrap().pretty_string())
    }
}

fn parse_args(matches: &clap::ArgMatches) -> (Box<dyn ComponentLoader>, Vec<Query>) {
    let folder_path = matches.value_of("folder").unwrap_or("");
    let query = matches.value_of("query").unwrap_or("");
    let settings = Settings {
        reduce_clocks_level: matches
            .value_of("clock-reduction-level")
            .map(|v| {
                let lvl = i32::from_str(v)
                    .unwrap_or_else(|e| panic!("Argument {} could not be parsed", e));
                if lvl < 0 {
                    ReduceClocksLevel::All(false)
                } else {
                    ReduceClocksLevel::Level(lvl)
                }
            })
            .or(DEFAULT_SETTINGS.reduce_clocks_level),
    };

    let project_loader = get_project_loader(folder_path.to_string(), settings);

    let queries = if query.is_empty() {
        project_loader.get_queries().clone()
    } else {
        parse_queries::parse_to_query(query)
    };

    (project_loader.to_comp_loader(), queries)
}

fn get_project_loader(project_path: String, settings: Settings) -> Box<dyn ProjectLoader> {
    if xml_parser::is_xml_project(&project_path) {
        XmlProjectLoader::new(project_path, settings)
    } else {
        JsonProjectLoader::new(project_path, settings)
    }
}

pub fn set_working_directory(folder_path: &str) {
    let mut path = std::path::Path::new(folder_path);
    if path.is_file() {
        path = path
            .parent()
            .expect("Failed to find parent directory of input file");
    };
    env::set_current_dir(path).expect("Failed to set working directory to input folder");
}
